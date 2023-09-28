use crate::sql::idiom::Idiom;
use crate::sql::value::{CowValue, Value};
use revision::revisioned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[serde(tag = "op")]
#[serde(rename_all = "lowercase")]
#[revisioned(revision = 1)]
pub enum Operation<'a> {
	Add {
		path: Idiom,
		value: CowValue<'a>,
	},
	Remove {
		path: Idiom,
	},
	Replace {
		path: Idiom,
		value: CowValue<'a>,
	},
	Change {
		path: Idiom,
		value: CowValue<'a>,
	},
	Copy {
		path: Idiom,
		from: Idiom,
	},
	Move {
		path: Idiom,
		from: Idiom,
	},
	Test {
		path: Idiom,
		value: CowValue<'a>,
	},
}
