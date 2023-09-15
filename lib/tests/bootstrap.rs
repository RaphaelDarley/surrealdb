/// The tests in this file are checking that bootstrapping of the database works correctly
/// They are testing edge cases that may accidentally occur with bugs - we wan't to make sure
/// the system can recover in light of these issues.
///
/// We may want to move these tests to another suite, as they aren't testing the statements like
/// the other tests are.
mod helpers;
mod parse;

use helpers::new_ds;
use surrealdb::err::Error;
use surrealdb::kvs::Transaction;
use surrealdb::sql::statements::LiveStatement;
use surrealdb::sql::Uuid;

#[tokio::test]
async fn bootstrap_removes_unreachable_nodes() -> Result<(), Error> {
	// Create the datastore
	let dbs = new_ds().await.unwrap();

	let mut tx = dbs.transaction(true, false).await.unwrap();
	// Introduce missing nodes (without heartbeats)
	let bad_node = uuid::Uuid::parse_str("9d8e16e4-9f6a-4704-8cf1-7cd55b937c5b").unwrap();
	tx.set_nd(bad_node).await.unwrap();

	// Introduce a valid chain of data to confirm it is not removed from a cleanup
	a_valid_notification(
		&mut tx,
		BootstrapPrerequisites {
			namesapce: "testns".to_string(),
			database: "testdb".to_string(),
			table: "testtb".to_string(),
		},
	)
	.await
	.unwrap();

	tx.commit().await.unwrap();

	// Bootstrap
	dbs.bootstrap().await.unwrap();

	// Verify the incorrect node is deleted, but self is inserted
	let mut tx = dbs.transaction(true, false).await.unwrap();
	let res = tx.scan_cl(1000).await.unwrap();
	tx.cancel().await.unwrap();
	assert_eq!(res.len(), 1);
	let cluster_membership = res.get(0).unwrap();
	assert_ne!(cluster_membership.name, bad_node.to_string());
	Ok(())
}
#[tokio::test]
async fn bootstrap_removes_unreachable_node_live_queries() -> Result<(), Error> {
	// Create the datastore
	let dbs = new_ds().await.unwrap();

	// Introduce an invalid node live query
	let mut tx = dbs.transaction(true, false).await.unwrap();
	let valid_data = a_valid_notification(
		&mut tx,
		BootstrapPrerequisites {
			namesapce: "testns".to_string(),
			database: "testdb".to_string(),
			table: "testtb".to_string(),
		},
	)
	.await
	.unwrap();
	let bad_nd_lq_id = uuid::Uuid::parse_str("67b0f588-2b95-4b6e-87f3-73d0a49034be").unwrap();
	tx.putc_ndlq(
		bad_nd_lq_id,
		valid_data.live_query_id.0,
		&valid_data.req.namesapce,
		&valid_data.req.database,
		&valid_data.req.table,
		None,
	)
	.await
	.unwrap();
	tx.commit().await.unwrap();

	// Bootstrap
	dbs.bootstrap().await.unwrap();

	// Verify node live query is deleted
	let mut tx = dbs.transaction(true, false).await.unwrap();
	let res = tx.scan_ndlq(&valid_data.node_id, 1000).await.unwrap();
	tx.cancel().await.unwrap();
	assert_eq!(res.len(), 1);
	let tested_entry = res.get(0).unwrap();
	assert_ne!(tested_entry.lq.0, bad_nd_lq_id);
	assert_eq!(tested_entry.lq, valid_data.live_query_id);

	Ok(())
}

#[tokio::test]
async fn bootstrap_removes_unreachable_table_live_queries() -> Result<(), Error> {
	// Create the datastore

	// Introduce a valid heartbeat

	// Introduce a valid node

	// Introduce a valid node live query

	// Introduce an invalid table live query

	// Introduce a valid table live query for coherency

	// Bootstrap

	// Verify invalid table live query is deleted
	Ok(())
}

#[tokio::test]
async fn bootstrap_removes_unreachable_live_query_notifications() -> Result<(), Error> {
	Ok(())
}

/// ValidBootstrapState is a representation of a chain of information that bootstrap is concerned
/// with. It is used for two reasons
/// - sometimes we want to detect invalid data that has a valid path (notification without a live query).
/// - sometimes we want to detect existing valid data is not deleted
#[derive(Debug, Clone)]
struct ValidBootstrapState {
	pub timestamp: u64,
	pub node_id: surrealdb::sql::Uuid,
	pub live_query_id: surrealdb::sql::Uuid,
	pub notification_id: surrealdb::sql::Uuid,
	pub req: BootstrapPrerequisites,
}

#[derive(Debug, Clone)]
struct BootstrapPrerequisites {
	pub namesapce: String,
	pub database: String,
	pub table: String,
}

/// Create a chain of valid state that bootstrapping should not remove.
async fn a_valid_notification(
	tx: &mut Transaction,
	args: BootstrapPrerequisites,
) -> Result<ValidBootstrapState, Error> {
	let now = tx.clock();
	let entry = ValidBootstrapState {
		timestamp: now.value,
		node_id: Uuid::from(uuid::Uuid::parse_str("123e9d92-c975-4daf-8080-3082e83cfa9b").unwrap()),
		live_query_id: Uuid::from(
			uuid::Uuid::parse_str("ca02c2d0-31dd-4bf0-ada4-ee02b1191e0a").unwrap(),
		),
		notification_id: Uuid::from(
			uuid::Uuid::parse_str("c952cf7d-b503-4370-802e-cd2404f2160d").unwrap(),
		),
		req: args.clone(),
	};
	// Create heartbeat
	tx.set_hb(entry.timestamp.into(), entry.node_id.0).await?;
	// Create cluster node entry
	tx.set_nd(entry.node_id.0).await?;
	// Create node live query registration
	tx.putc_ndlq(
		entry.node_id.0,
		entry.live_query_id.0,
		&args.namesapce,
		&args.database,
		&args.table,
		None,
	)
	.await?;
	// Create table live query registration
	let mut live_stm = LiveStatement::default();
	live_stm.id = entry.live_query_id.clone().into();
	live_stm.node = entry.node_id.clone().into();
	tx.putc_tblq(&args.namesapce, &args.database, &args.table, live_stm, None).await?;
	// TODO Create notification
	// tx.putc_nt(
	// 	entry.notification_id,
	// 	entry.timestamp.into(),
	// 	entry.node_id,
	// 	&args.namesapce,
	// 	&args.database,
	// 	&args.table,
	// 	None,
	// ).await?;
	Ok(entry)
}
