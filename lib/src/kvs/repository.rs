use crate::{
	err::Error,
	kvs::Transaction,
	sql::{statements::DefineFieldStatement, Idiom, Kind, Part, Relation},
};

pub(crate) struct TableRepository<'a> {
	tx: &'a mut Transaction,
	ns: &'a str,
	db: &'a str,
	tb: &'a str,
}
impl<'a> TableRepository<'a> {
	pub fn new(tx: &'a mut Transaction, ns: &'a str, db: &'a str, tb: &'a str) -> Self {
		Self {
			tx,
			ns,
			db,
			tb,
		}
	}
	pub async fn define_in_out_fd_from_relation(&mut self, rel: &Relation) -> Result<(), Error> {
		let in_kind = rel.from.clone().unwrap_or(Kind::Record(vec![]));
		let out_kind = rel.to.clone().unwrap_or(Kind::Record(vec![]));

		let in_key = crate::key::table::fd::new(self.ns, self.db, self.tb, "in");
		let out_key = crate::key::table::fd::new(self.ns, self.db, self.tb, "out");

		// TODO: fix permissions so they don't defalut to full
		self.tx
			.set(
				in_key,
				DefineFieldStatement {
					name: Idiom(vec![Part::from("in")]),
					what: self.tb.into(),
					kind: Some(in_kind),
					..Default::default()
				},
			)
			.await?;
		self.tx
			.set(
				out_key,
				DefineFieldStatement {
					name: Idiom(vec![Part::from("out")]),
					what: self.tb.into(),
					kind: Some(out_kind),
					..Default::default()
				},
			)
			.await?;
		Ok(())
	}
}
