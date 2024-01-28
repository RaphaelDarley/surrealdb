use std::collections::BTreeMap;

use crate::err::Error;
use crate::sql;
use crate::sql::statements::SelectStatement;
use crate::sql::Fields;
use crate::sql::Limit;
use crate::sql::Query;
use crate::sql::Start;
use crate::sql::Statement;
use crate::sql::Table;
use crate::sql::Values;

pub fn parse_and_transpile(txt: &str) -> Result<Query, Error> {
	todo!()
}
