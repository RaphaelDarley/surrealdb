use crate::{
	sql::{statements::InsertStatement, Data, Ident, Param, Table, Value},
	syn::{
		parser::{mac::expected, ParseResult, Parser},
		token::t,
	},
};

impl Parser<'_> {
	pub(crate) fn parse_insert_stmt(&mut self) -> ParseResult<InsertStatement> {
		let ignore = self.eat(t!("IGNORE"));
		expected!(self, "INTO");
		let next = self.peek();
		let into = match self.next().kind {
			t!("$param") => {
				self.pop_peek();
				let str = self.lexer.strings[u32::from(next.data_index.unwrap()) as usize].clone();
				Value::Param(Param(Ident(str)))
			}
			_ => {
				let table = self.parse_raw_ident()?;
				Value::Table(Table(table))
			}
		};

		let data = match self.peek().kind {
			t!("(") => {
				let start = self.pop_peek().span;
				let fields = self.parse_idiom_list()?;
				self.expect_closing_delimiter(t!(")"), start)?;
				expected!(self, "VALUES");

				let start = expected!(self, "(").span;
				let mut values = vec![self.parse_value()?];
				while self.eat(t!(",")) {
					values.push(self.parse_value()?);
				}
				self.expect_closing_delimiter(t!(")"), start)?;

				let mut values = vec![values];
				while self.eat(t!(",")) {
					let start = expected!(self, "(").span;
					let mut inner_values = vec![self.parse_value()?];
					while self.eat(t!(",")) {
						inner_values.push(self.parse_value()?);
					}
					values.push(inner_values);
					self.expect_closing_delimiter(t!(")"), start)?;
				}

				Data::ValuesExpression(
					values
						.into_iter()
						.map(|row| fields.iter().cloned().zip(row).collect())
						.collect(),
				)
			}
			_ => {
				let value = self.parse_value()?;
				Data::SingleExpression(value)
			}
		};

		let update = self.eat(t!("ON")).then(|| self.parse_insert_update()).transpose()?;
		let output = self.try_parse_output()?;
		let timeout = self.try_parse_timeout()?;
		let parallel = self.eat(t!("PARALLEL"));
		Ok(InsertStatement {
			into,
			data,
			ignore,
			update,
			output,
			timeout,
			parallel,
		})
	}

	fn parse_insert_update(&mut self) -> ParseResult<Data> {
		expected!(self, "DUPLICATE");
		expected!(self, "KEY");
		expected!(self, "UPDATE");
		let l = self.parse_plain_idiom()?;
		let o = self.parse_assigner()?;
		let r = self.parse_value()?;
		let mut data = vec![(l, o, r)];

		while self.eat(t!(",")) {
			let l = self.parse_plain_idiom()?;
			let o = self.parse_assigner()?;
			let r = self.parse_value()?;
			data.push((l, o, r))
		}

		Ok(Data::UpdateExpression(data))
	}
}