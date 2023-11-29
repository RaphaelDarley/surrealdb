use geo::Point;

use super::{ParseResult, Parser};
use crate::{
	sql::{Array, Dir, Geometry, Ident, Idiom, Mock, Number, Part, Subquery, Table, Value},
	syn::v2::{
		parser::{
			mac::{expected, to_do},
			ParseError, ParseErrorKind,
		},
		token::{t, Span, TokenKind},
	},
};

impl Parser<'_> {
	/// Parse a what primary.
	///
	/// What's are values which are more restricted in what expressions they can contain.
	pub fn parse_what_primary(&mut self) -> ParseResult<Value> {
		match self.peek_kind() {
			TokenKind::Duration => {
				let duration = self.parse_token_value()?;
				Ok(Value::Duration(duration))
			}
			TokenKind::DateTime => {
				let datetime = self.parse_token_value()?;
				Ok(Value::Datetime(datetime))
			}
			t!("r\"") => {
				self.pop_peek();
				Ok(Value::Thing(self.parse_record_string(true)?))
			}
			t!("r'") => {
				self.pop_peek();
				Ok(Value::Thing(self.parse_record_string(false)?))
			}
			t!("$param") => {
				let param = self.parse_token_value()?;
				Ok(Value::Param(param))
			}
			t!("IF") => {
				let stmt = self.parse_if_stmt()?;
				Ok(Value::Subquery(Box::new(Subquery::Ifelse(stmt))))
			}
			t!("(") => {
				let token = self.pop_peek();
				self.parse_inner_subquery(Some(token.span)).map(|x| Value::Subquery(Box::new(x)))
			}
			t!("<") => {
				self.pop_peek();
				expected!(self, "FUTURE");
				expected!(self, ">");
				let start = expected!(self, "{").span;
				let block = self.parse_block(start)?;
				Ok(Value::Future(Box::new(crate::sql::Future(block))))
			}
			t!("|") => {
				let start = self.pop_peek().span;
				self.parse_mock(start).map(Value::Mock)
			}
			t!("/") => {
				let token = self.pop_peek();
				let regex = self.lexer.relex_regex(token);
				self.token_value(regex).map(Value::Regex)
			}
			t!("RETURN")
			| t!("SELECT")
			| t!("CREATE")
			| t!("UPDATE")
			| t!("DELETE")
			| t!("RELATE")
			| t!("DEFINE")
			| t!("REMOVE") => self.parse_inner_subquery(None).map(|x| Value::Subquery(Box::new(x))),
			t!("fn") => self.parse_custom_function().map(|x| Value::Function(Box::new(x))),
			t!("ml") => self.parse_model().map(|x| Value::MlModel(Box::new(x))),
			_ => {
				let token = self.next();
				match self.peek_kind() {
					t!("::") | t!("(") => self.parse_builtin(token.span),
					t!(":") => {
						let str = self.token_value::<Ident>(token)?.0;
						self.parse_thing_or_range(str)
					}
					_ => Ok(Value::Table(self.token_value(token)?)),
				}
			}
		}
	}

	/// Parse an expressions
	pub fn parse_idiom_expression(&mut self) -> ParseResult<Value> {
		let token = self.peek();
		let value = match token.kind {
			t!("NONE") => {
				self.pop_peek();
				return Ok(Value::None);
			}
			t!("NULL") => {
				self.pop_peek();
				return Ok(Value::Null);
			}
			t!("true") => {
				self.pop_peek();
				return Ok(Value::Bool(true));
			}
			t!("false") => {
				self.pop_peek();
				return Ok(Value::Bool(false));
			}
			t!("<") => {
				self.pop_peek();
				// Casting should already have been parsed.
				expected!(self, "FUTURE");
				self.expect_closing_delimiter(t!(">"), token.span)?;
				let next = expected!(self, "{").span;
				let block = self.parse_block(next)?;
				return Ok(Value::Future(Box::new(crate::sql::Future(block))));
			}
			TokenKind::Strand => {
				self.pop_peek();
				let strand = self.token_value(token)?;
				return Ok(Value::Strand(strand));
			}
			TokenKind::Duration => {
				self.pop_peek();
				let duration = self.token_value(token)?;
				Value::Duration(duration)
			}
			TokenKind::Number => {
				self.pop_peek();
				let number = self.token_value(token)?;
				Value::Number(number)
			}
			TokenKind::Uuid => {
				self.pop_peek();
				let uuid = self.token_value(token)?;
				Value::Uuid(uuid)
			}
			TokenKind::DateTime => {
				self.pop_peek();
				let datetime = self.token_value(token)?;
				Value::Datetime(datetime)
			}
			t!("r\"") => {
				self.pop_peek();
				Value::Thing(self.parse_record_string(true)?)
			}
			t!("r'") => {
				self.pop_peek();
				Value::Thing(self.parse_record_string(false)?)
			}
			t!("$param") => {
				self.pop_peek();
				let param = self.token_value(token)?;
				Value::Param(param)
			}
			t!("FUNCTION") => {
				to_do!(self)
			}
			t!("->") => {
				self.pop_peek();
				let graph = self.parse_graph(Dir::Out)?;
				Value::Idiom(Idiom(vec![Part::Graph(graph)]))
			}
			t!("<->") => {
				self.pop_peek();
				let graph = self.parse_graph(Dir::Both)?;
				Value::Idiom(Idiom(vec![Part::Graph(graph)]))
			}
			t!("<-") => {
				self.pop_peek();
				let graph = self.parse_graph(Dir::In)?;
				Value::Idiom(Idiom(vec![Part::Graph(graph)]))
			}
			t!("[") => {
				self.pop_peek();
				self.parse_array(token.span).map(Value::Array)?
			}
			t!("{") => {
				self.pop_peek();
				self.parse_object_like(token.span)?
			}
			t!("|") => {
				self.pop_peek();
				self.parse_mock(token.span).map(Value::Mock)?
			}
			t!("IF") => {
				self.pop_peek();
				let stmt = self.parse_if_stmt()?;
				Value::Subquery(Box::new(Subquery::Ifelse(stmt)))
			}
			t!("(") => {
				self.pop_peek();
				self.parse_inner_subquery_or_coordinate(token.span)?
			}
			t!("/") => {
				self.pop_peek();
				let regex = self.lexer.relex_regex(token);
				self.token_value(regex).map(Value::Regex)?
			}
			t!("RETURN")
			| t!("SELECT")
			| t!("CREATE")
			| t!("UPDATE")
			| t!("DELETE")
			| t!("RELATE")
			| t!("DEFINE")
			| t!("REMOVE") => self.parse_inner_subquery(None).map(|x| Value::Subquery(Box::new(x)))?,
			t!("fn") => {
				self.pop_peek();
				self.parse_custom_function().map(|x| Value::Function(Box::new(x)))?
			}
			t!("ml") => {
				self.pop_peek();
				self.parse_model().map(|x| Value::MlModel(Box::new(x)))?
			}
			_ => {
				self.pop_peek();
				match self.peek_kind() {
					t!("::") | t!("(") => self.parse_builtin(token.span)?,
					t!(":") => {
						let str = self.token_value::<Ident>(token)?.0;
						self.parse_thing_or_range(str)?
					}
					_ => {
						if self.table_as_field {
							Value::Idiom(Idiom(vec![Part::Field(self.token_value(token)?)]))
						} else {
							Value::Table(self.token_value(token)?)
						}
					}
				}
			}
		};

		// Parse the rest of the idiom if it is being continued.
		if Self::continues_idiom(self.peek_kind()) {
			match value {
				Value::None
				| Value::Null
				| Value::Bool(_)
				| Value::Future(_)
				| Value::Strand(_) => unreachable!(),
				Value::Idiom(Idiom(x)) => self.parse_remaining_value_idiom(x),
				Value::Table(Table(x)) => {
					self.parse_remaining_value_idiom(vec![Part::Field(Ident(x))])
				}
				x => self.parse_remaining_value_idiom(vec![Part::Start(x)]),
			}
		} else {
			Ok(value)
		}
	}

	/// Parses an array production
	///
	/// # Parser state
	/// Expects the starting `[` to already be eaten and its span passed as an argument.
	pub fn parse_array(&mut self, start: Span) -> ParseResult<Array> {
		let mut values = Vec::new();
		loop {
			if self.eat(t!("]")) {
				break;
			}
			values.push(self.parse_value_field()?);

			if !self.eat(t!(",")) {
				self.expect_closing_delimiter(t!("]"), start)?;
				break;
			}
		}

		Ok(Array(values))
	}

	pub fn parse_mock(&mut self, start: Span) -> ParseResult<Mock> {
		let name = self.parse_token_value::<Ident>()?.0;
		expected!(self, ":");
		let from = self.parse_token_value()?;
		let to = self.eat(t!("..")).then(|| self.parse_token_value()).transpose()?;
		self.expect_closing_delimiter(t!("|"), start)?;
		if let Some(to) = to {
			Ok(Mock::Range(name, from, to))
		} else {
			Ok(Mock::Count(name, from))
		}
	}

	pub fn parse_full_subquery(&mut self) -> ParseResult<Subquery> {
		let peek = self.peek();
		match peek.kind {
			t!("(") => {
				self.pop_peek();
				self.parse_inner_subquery(Some(peek.span))
			}
			t!("IF") => {
				self.pop_peek();
				let if_stmt = self.parse_if_stmt()?;
				Ok(Subquery::Ifelse(if_stmt))
			}
			_ => self.parse_inner_subquery(None),
		}
	}

	pub fn parse_inner_subquery_or_coordinate(&mut self, start: Span) -> ParseResult<Value> {
		let next = self.peek();
		let res = match next.kind {
			t!("RETURN") => {
				self.pop_peek();
				let stmt = self.parse_return_stmt()?;
				Subquery::Output(stmt)
			}
			t!("SELECT") => {
				self.pop_peek();
				let stmt = self.parse_select_stmt()?;
				Subquery::Select(stmt)
			}
			t!("CREATE") => {
				self.pop_peek();
				let stmt = self.parse_create_stmt()?;
				Subquery::Create(stmt)
			}
			t!("UPDATE") => {
				self.pop_peek();
				let stmt = self.parse_update_stmt()?;
				Subquery::Update(stmt)
			}
			t!("DELETE") => {
				self.pop_peek();
				let stmt = self.parse_delete_stmt()?;
				Subquery::Delete(stmt)
			}
			t!("RELATE") => {
				self.pop_peek();
				let stmt = self.parse_relate_stmt()?;
				Subquery::Relate(stmt)
			}
			t!("DEFINE") => {
				self.pop_peek();
				let stmt = self.parse_define_stmt()?;
				Subquery::Define(stmt)
			}
			t!("REMOVE") => {
				self.pop_peek();
				let stmt = self.parse_remove_stmt()?;
				Subquery::Remove(stmt)
			}
			_ => {
				let value = self.parse_value_field()?;
				Subquery::Value(value)
			}
		};
		match res {
			Subquery::Value(Value::Number(x)) => {
				if self.eat(t!(",")) {
					// TODO: Fix number parsing.
					let b = self.parse_token_value::<Number>()?;

					let a: f64 = x
						.try_into()
						.map_err(|_| ParseError::new(ParseErrorKind::Todo, next.span))?;
					let b: f64 = b
						.try_into()
						.map_err(|_| ParseError::new(ParseErrorKind::Todo, next.span))?;

					self.expect_closing_delimiter(t!(")"), start)?;
					Ok(Value::Geometry(Geometry::Point(Point::from((a, b)))))
				} else {
					Ok(Value::Subquery(Box::new(Subquery::Value(Value::Number(x)))))
				}
			}
			x => {
				self.expect_closing_delimiter(t!(")"), start)?;
				Ok(Value::Subquery(Box::new(x)))
			}
		}
	}

	pub fn parse_inner_subquery(&mut self, start: Option<Span>) -> ParseResult<Subquery> {
		let res = match self.peek().kind {
			t!("RETURN") => {
				self.pop_peek();
				let stmt = self.parse_return_stmt()?;
				Subquery::Output(stmt)
			}
			t!("SELECT") => {
				self.pop_peek();
				let stmt = self.parse_select_stmt()?;
				Subquery::Select(stmt)
			}
			t!("CREATE") => {
				self.pop_peek();
				let stmt = self.parse_create_stmt()?;
				Subquery::Create(stmt)
			}
			t!("UPDATE") => {
				self.pop_peek();
				let stmt = self.parse_update_stmt()?;
				Subquery::Update(stmt)
			}
			t!("DELETE") => {
				self.pop_peek();
				let stmt = self.parse_delete_stmt()?;
				Subquery::Delete(stmt)
			}
			t!("RELATE") => {
				self.pop_peek();
				let stmt = self.parse_relate_stmt()?;
				Subquery::Relate(stmt)
			}
			t!("DEFINE") => {
				self.pop_peek();
				let stmt = self.parse_define_stmt()?;
				Subquery::Define(stmt)
			}
			t!("REMOVE") => {
				self.pop_peek();
				let stmt = self.parse_remove_stmt()?;
				Subquery::Remove(stmt)
			}
			_ => {
				let value = self.parse_value_field()?;
				Subquery::Value(value)
			}
		};
		if let Some(start) = start {
			self.expect_closing_delimiter(t!(")"), start)?;
		}
		Ok(res)
	}
}