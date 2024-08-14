use reblessive::Stk;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::{ctx::Context, dbs::Options, doc::CursorDoc, err::Error};

use super::Value;

#[revisioned(revision = 1)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "$surrealdb::private::sql::Specialized")]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub enum Specialized {
	F32Array(Vec<f32>),
	F64Array(Vec<f64>),
	U64Array(Vec<u64>),
}

impl Hash for Specialized {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		core::mem::discriminant(self).hash(state);
		match self {
			Specialized::F32Array(v) => {
				for f in v {
					state.write(&f.to_ne_bytes())
				}
			}
			Specialized::F64Array(v) => {
				for f in v {
					state.write(&f.to_ne_bytes())
				}
			}
			Specialized::U64Array(v) => v.hash(state),
		}
	}
}

impl Specialized {
	/// Process this type returning a computed simple Value
	pub(crate) async fn compute(
		&self,
		_stk: &mut Stk,
		_ctx: &Context<'_>,
		_opt: &Options,
		_doc: Option<&CursorDoc<'_>>,
	) -> Result<Value, Error> {
		Ok(match self {
			Specialized::F32Array(v) => Value::Array(v.to_owned().into()),
			Specialized::F64Array(v) => todo!(),
			Specialized::U64Array(v) => todo!(),
		})
	}
}
