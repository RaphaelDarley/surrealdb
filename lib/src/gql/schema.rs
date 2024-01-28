use async_graphql::Schema;

use crate::err::Error;
use crate::kvs::Datastore;

pub async fn get_schema(ds: &Datastore, ns: String, db: String) -> Result<String, Error> {
	todo!()
}
