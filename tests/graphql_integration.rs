mod common;

#[cfg(surrealdb_unstable)]
mod graphql_integration {
	use std::time::Duration;

	use http::header;
	use reqwest::Client;
	use serde_json::json;
	use surrealdb::headers::{AUTH_DB, AUTH_NS};
	use surrealdb::sql;
	use test_log::test;
	use tracing::debug;
	use ulid::Ulid;

	use crate::common::{PASS, USER};

	use super::common::{self};

	#[test(tokio::test)]
	async fn basic() -> Result<(), Box<dyn std::error::Error>> {
		let (addr, _server) = common::start_server_gql_without_auth().await.unwrap();
		let gql_url = &format!("http://{addr}/graphql");
		let sql_url = &format!("http://{addr}/sql");

		let mut headers = reqwest::header::HeaderMap::new();
		let ns = Ulid::new().to_string();
		let db = Ulid::new().to_string();
		headers.insert("surreal-ns", ns.parse()?);
		headers.insert("surreal-db", db.parse()?);
		headers.insert(header::ACCEPT, "application/json".parse()?);
		let client = Client::builder()
			.connect_timeout(Duration::from_millis(10))
			.default_headers(headers)
			.build()?;

		// check errors with no tables
		{
			let res = client.post(gql_url).body("").send().await?;
			assert_eq!(res.status(), 400);
			let body = res.text().await?;
			assert!(body.contains("no tables found in database"), "body: {body}")
		}

		// add schema and data
		{
			let res = client
				.post(sql_url)
				.body(
					r#"
                    DEFINE TABLE foo SCHEMAFUL;
                    DEFINE FIELD val ON foo TYPE int;
                    CREATE foo:1 set val = 42;
                    CREATE foo:2 set val = 43;
                "#,
				)
				.send()
				.await?;
			assert_eq!(res.status(), 200);
		}

		// fetch data via graphql
		{
			let res = client
				.post(gql_url)
				.body(json!({"query": r#"query{foo{id, val}}"#}).to_string())
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({
				"data": {
					"foo": [
						{
							"id": "foo:1",
							"val": 42
						},
						{
							"id": "foo:2",
							"val": 43
						}
					]
				}
			});
			assert_eq!(expected.to_string(), body)
		}

		// test limit
		{
			let res = client
				.post(gql_url)
				.body(json!({"query": r#"query{foo(limit: 1){id, val}}"#}).to_string())
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({
				"data": {
					"foo": [
						{
							"id": "foo:1",
							"val": 42
						}
					]
				}
			});
			assert_eq!(expected.to_string(), body)
		}

		// test start
		{
			let res = client
				.post(gql_url)
				.body(json!({"query": r#"query{foo(start: 1){id, val}}"#}).to_string())
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({
				"data": {
					"foo": [
						{
							"id": "foo:2",
							"val": 43
						}
					]
				}
			});
			assert_eq!(expected.to_string(), body)
		}

		// test order
		{
			let res = client
				.post(gql_url)
				.body(json!({"query": r#"query{foo(order: {desc: val}){id}}"#}).to_string())
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({
				"data": {
					"foo": [
						{
							"id": "foo:2",
						},
						{
							"id": "foo:1",
						}
					]
				}
			});
			assert_eq!(expected.to_string(), body)
		}

		// test filter
		{
			let res = client
				.post(gql_url)
				.body(json!({"query": r#"query{foo(filter: {val: {eq: 42}}){id}}"#}).to_string())
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({
				"data": {
					"foo": [
						{
							"id": "foo:1",
						}
					]
				}
			});
			assert_eq!(expected.to_string(), body)
		}

		Ok(())
	}

	#[test(tokio::test)]
	async fn basic_auth() -> Result<(), Box<dyn std::error::Error>> {
		let (addr, _server) = common::start_server_gql().await.unwrap();
		let gql_url = &format!("http://{addr}/graphql");
		let sql_url = &format!("http://{addr}/sql");
		let signup_url = &format!("http://{addr}/signup");

		let mut headers = reqwest::header::HeaderMap::new();
		let ns = Ulid::new().to_string();
		let db = Ulid::new().to_string();
		headers.insert("surreal-ns", ns.parse()?);
		headers.insert("surreal-db", db.parse()?);
		headers.insert(header::ACCEPT, "application/json".parse()?);
		let client = Client::builder()
			.connect_timeout(Duration::from_millis(10))
			.default_headers(headers)
			.build()?;

		// check errors on invalid auth
		{
			let res =
				client.post(gql_url).basic_auth("invalid", Some("invalid")).body("").send().await?;
			assert_eq!(res.status(), 401);
			let body = res.text().await?;
			assert!(body.contains("There was a problem with authentication"), "body: {body}")
		}

		// add schema and data
		{
			let res = client
				.post(sql_url)
				.basic_auth(USER, Some(PASS))
				.body(
					r#"
					DEFINE ACCESS user1 ON DATABASE TYPE RECORD
					SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
					SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
					DURATION FOR SESSION 60s, FOR TOKEN 1d;

					DEFINE ACCESS user2 ON DATABASE TYPE RECORD
					SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
					SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
					DURATION FOR SESSION 60s, FOR TOKEN 1d;

                    DEFINE TABLE foo SCHEMAFUL;
                    DEFINE FIELD val1 ON foo TYPE int PERMISSIONS FOR select WHERE $access = "user1" || $access = "user2";
                    DEFINE FIELD val2 ON foo TYPE int PERMISSIONS FOR select WHERE $access = "user2";
                    CREATE foo:1 set val1 = 42, val2 = 100;
                "#,
				)
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			// let body = res.text().await?;
			// panic!("{body:?}")
		}

		// check works with root
		{
			let res = client
				.post(gql_url)
				.basic_auth(USER, Some(PASS))
				.body(
					json!({"query": r#"query{_get_foo(id: "foo:1"){id, val1, val2}}"#}).to_string(),
				)
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({"data":{"_get_foo":{"id":"foo:1","val1":42,"val2":100}}});
			assert_eq!(expected.to_string(), body);
		}

		// check partial access
		{
			let req_body = serde_json::to_string(
				json!({
					"ns": ns,
					"db": db,
					"ac": "user1",
					"email": "email@email.com",
					"pass": "pass",
				})
				.as_object()
				.unwrap(),
			)
			.unwrap();

			let res = client.post(signup_url).body(req_body).send().await?;
			assert_eq!(res.status(), 200, "body: {}", res.text().await?);
			let body: serde_json::Value = serde_json::from_str(&res.text().await?).unwrap();
			let token = body["token"].as_str().unwrap();

			let res = client
				.post(gql_url)
				.bearer_auth(token)
				.body(
					json!({"query": r#"query{_get_foo(id: "foo:1"){id, val1, val2}}"#}).to_string(),
				)
				.send()
				.await?;
			assert_eq!(res.status(), 200);
			let body = res.text().await?;
			let expected = json!({"data":{"_get_foo":{"id":"foo:1","val1":42,"val2":100}}});
			assert_eq!(expected.to_string(), body);
		}
		Ok(())
	}
}
