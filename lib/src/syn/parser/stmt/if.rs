use crate::{
	sql::statements::IfelseStatement,
	syn::{
		parser::{
			mac::{expected, unexpected},
			ParseResult, Parser,
		},
		token::t,
	},
};

impl Parser<'_> {
	pub fn parse_if_stmt(&mut self) -> ParseResult<IfelseStatement> {
		let condition = self.parse_value_field()?;

		let mut res = IfelseStatement {
			exprs: Vec::new(),
			close: None,
		};

		let next = self.next();
		match next.kind {
			t!("THEN") => {
				let body = self.parse_value_field()?;
				res.exprs.push((condition, body));
				self.parse_worded_tail(&mut res)?;
			}
			t!("{") => {
				let body = self.parse_block(next.span)?;
				res.exprs.push((condition, body.into()));
				self.parse_bracketed_tail(&mut res)?;
			}
			x => unexpected!(self, x, "THEN or '{'"),
		}

		Ok(res)
	}

	fn parse_worded_tail(&mut self, res: &mut IfelseStatement) -> ParseResult<()> {
		loop {
			match self.next().kind {
				t!("END") => return Ok(()),
				t!("ELSE") => {
					if self.eat(t!("IF")) {
						let condition = self.parse_value_field()?;
						expected!(self, "THEN");
						let body = self.parse_value_field()?;
						res.exprs.push((condition, body));
					} else {
						let value = self.parse_value_field()?;
						expected!(self, "END");
						res.close = Some(value);
						return Ok(());
					}
				}
				x => unexpected!(self, x, "if to end"),
			}
		}
	}

	fn parse_bracketed_tail(&mut self, res: &mut IfelseStatement) -> ParseResult<()> {
		loop {
			match self.next().kind {
				t!("ELSE") => {
					if self.eat(t!("IF")) {
						let condition = self.parse_value_field()?;
						let span = expected!(self, "{").span;
						let body = self.parse_block(span)?;
						res.exprs.push((condition, body.into()));
					} else {
						let span = expected!(self, "{").span;
						let value = self.parse_block(span)?;
						res.close = Some(value.into());
						return Ok(());
					}
				}
				_ => return Ok(()),
			}
		}
	}
}
