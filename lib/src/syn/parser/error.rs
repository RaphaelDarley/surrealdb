use crate::{
	sql::error::{Location, RenderedError, Snippet},
	syn::token::{Span, TokenKind},
};

#[derive(Debug)]
pub enum ParseErrorKind {
	/// The parser encountered an unexpected token.
	Unexpected {
		found: TokenKind,
		expected: &'static str,
	},
	/// The parser encountered an unexpected token.
	UnexpectedEof {
		expected: &'static str,
	},
	UnclosedDelimiter {
		expected: TokenKind,
		should_close: Span,
	},
	Retried {
		first: Box<ParseError>,
		then: Box<ParseError>,
	},
	DisallowedStatement,
	/// The parser encountered an token which could not be lexed correctly.
	InvalidToken,
	/// A path in the parser which was not yet finished.
	/// Should eventually be removed.
	Todo,
}

#[derive(Debug)]
pub struct ParseError {
	pub kind: ParseErrorKind,
	pub at: Span,
	pub backtrace: std::backtrace::Backtrace,
}

impl ParseError {
	pub fn new(kind: ParseErrorKind, at: Span) -> Self {
		ParseError {
			kind,
			at,
			backtrace: std::backtrace::Backtrace::force_capture(),
		}
	}

	pub fn render_on(&self, source: &str) -> RenderedError {
		println!("FOUND ERROR: {}", self.backtrace);
		match self.kind {
			ParseErrorKind::Unexpected {
				found,
				expected,
			} => {
				let text = format!("Unexpected token '{}' expected {}", found.as_str(), expected);
				let locations = Location::range_of_span(source, self.at);
				let snippet = Snippet::from_source_location_range(source, locations, None);
				RenderedError {
					text,
					snippets: vec![snippet],
				}
			}
			ParseErrorKind::UnexpectedEof {
				expected,
			} => {
				let text = format!("Query ended early, expected {}", expected);
				let locations = Location::range_of_span(source, self.at);
				let snippet = Snippet::from_source_location_range(source, locations, None);
				RenderedError {
					text,
					snippets: vec![snippet],
				}
			}
			ParseErrorKind::UnclosedDelimiter {
				expected,
				should_close,
			} => {
				let text = format!("Expected closing delimiter {}", expected.as_str());
				let locations = Location::range_of_span(source, self.at);
				let snippet = Snippet::from_source_location_range(source, locations, None);
				let locations = Location::range_of_span(source, should_close);
				let close_snippet = Snippet::from_source_location_range(
					source,
					locations,
					Some("Expected this delimiter to close"),
				);
				RenderedError {
					text,
					snippets: vec![snippet, close_snippet],
				}
			}
			ParseErrorKind::Retried {
				first,
				then,
			} => todo!(),
			ParseErrorKind::DisallowedStatement => todo!(),
			ParseErrorKind::InvalidToken => {
				let text = format!("Could not parse invalid token");
				let locations = Location::range_of_span(source, self.at);
				let snippet = Snippet::from_source_location_range(source, locations, None);
				RenderedError {
					text,
					snippets: vec![snippet],
				}
			}
			ParseErrorKind::Todo => {
				let text = format!("Parser hit not yet implemented path");
				let locations = Location::range_of_span(source, self.at);
				let snippet = Snippet::from_source_location_range(source, locations, None);
				RenderedError {
					text,
					snippets: vec![snippet],
				}
			}
		}
	}
}