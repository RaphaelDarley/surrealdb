use crate::ctx::Context;
use crate::dbs::{Options, Transaction};
use crate::doc::CursorDoc;
use crate::err::Error;
use crate::sql::common::openbracket;
use crate::sql::error::IResult;
use crate::sql::fmt::{pretty_indent, Fmt, Pretty};
use crate::sql::number::Number;
use crate::sql::operation::Operation;
use crate::sql::value::{CowValue, value, Value};
use nom::character::complete::char;
use nom::sequence::terminated;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter, Write};
use std::ops;
use std::ops::Deref;
use std::ops::DerefMut;

use super::comment::mightbespace;
use super::common::commas;
use super::util::delimited_list0;

pub(crate) const TOKEN: &str = "$surrealdb::private::sql::Array";

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[serde(rename = "$surrealdb::private::sql::Array")]
#[revisioned(revision = 1)]
pub struct Array<'a>(pub Vec<CowValue<'a>>);

impl FromIterator<Value> for Vec<Cow<'_, Value>> {
	fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
		iter.into_iter().map(|i| Cow::Owned(i)).collect()
	}
}

impl From<Value> for Array<'_> {
	fn from(v: Value) -> Self {
		vec![v].into()
	}
}

impl<'a> From<Vec<Cow<'a, Value>>> for Array<'a> {
	fn from(v: Vec<CowValue>) -> Self {
		Self(v)
	}
}

impl From<Vec<Value>> for Array<'_> {
	fn from(v: Vec<Value>) -> Self {
		Self(v.into_iter().map(|v| v.into()).collect())
	}
}

impl From<Vec<i32>> for Array<'_> {
	fn from(v: Vec<i32>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl From<Vec<f64>> for Array<'_> {
	fn from(v: Vec<f64>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl From<Vec<&str>> for Array<'_> {
	fn from(v: Vec<&str>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl From<Vec<String>> for Array<'_> {
	fn from(v: Vec<String>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl From<Vec<Number>> for Array<'_> {
	fn from(v: Vec<Number>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl From<Vec<Operation>> for Array<'_> {
	fn from(v: Vec<Operation>) -> Self {
		Self(v.into_iter().map(Value::from).collect())
	}
}

impl<'a> From<Array<'a>> for Vec<CowValue<'a>> {
	fn from(s: Array) -> Self {
		s.0
	}
}

impl FromIterator<Value> for Array<'_> {
	fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
		Array(iter.into_iter().collect())
	}
}

impl<'a> Deref for Array<'a> {
	type Target = Vec<CowValue<'a>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> DerefMut for Array<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<'a> IntoIterator for Array<'a> {
	type Item = CowValue<'a>;
	type IntoIter = std::vec::IntoIter<Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl Array<'_> {
	// Create a new empty array
	pub fn new() -> Self {
		Self::default()
	}
	// Create a new array with capacity
	pub fn with_capacity(len: usize) -> Self {
		Self(Vec::with_capacity(len))
	}
	// Get the length of the array
	pub fn len(&self) -> usize {
		self.0.len()
	}
	// Check if there array is empty
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl<'a> Array<'a> {
	/// Process this type returning a computed simple Value
	pub(crate) async fn compute(
		&self,
		ctx: &Context<'_>,
		opt: &Options,
		txn: &Transaction,
		doc: Option<&CursorDoc<'_>>,
	) -> Result<Value, Error> {
		let mut acc: Vec<Cow<'_, Value>> = Vec::with_capacity(self.len());
		for v in self.iter() {
			match v.compute(ctx, opt, txn, doc).await {
				Ok(v) => acc.push(Cow::Owned(v)),
				Err(e) => return Err(e),
			}
		}
		let arr: Array = acc.into();
		Ok(arr.into())
	}

	pub(crate) fn is_all_none_or_null(&self) -> bool {
		self.0.iter().all(|v| v.is_none_or_null())
	}
}

impl<'a> Display for Array<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut f = Pretty::from(f);
		f.write_char('[')?;
		if !self.is_empty() {
			let indent = pretty_indent();
			write!(f, "{}", Fmt::pretty_comma_separated(self.as_slice()))?;
			drop(indent);
		}
		f.write_char(']')
	}
}

// ------------------------------

impl<'a> ops::Add<Cow<'a, Value>> for Array<'a> {
	type Output = Self;
	fn add(mut self, other: CowValue<'a>) -> Self {
		self.0.push(other);
		self
	}
}

impl<'a> ops::Add<Value> for Array<'a> {
	type Output = Self;
	fn add(mut self, other: Value) -> Self {
		self.0.push(other.into());
		self
	}
}

impl<'a> ops::Add for Array<'a> {
	type Output = Self;
	fn add(mut self, mut other: Self) -> Self {
		self.0.append(&mut other.0);
		self
	}
}

// ------------------------------

impl<'a> ops::Sub<CowValue<'a>> for Array<'a> {
	type Output = Self;
	fn sub(mut self, other: CowValue<'a>) -> Self {
		if let Some(p) = self.0.iter().position(|x| *x == other) {
			self.0.remove(p);
		}
		self
	}
}

impl<'a> ops::Sub<Value> for Array<'a> {
	type Output = Self;
	fn sub(mut self, other: Value) -> Self {
		if let Some(p) = self.0.iter().position(|x| *x.as_ref() == other) {
			self.0.remove(p);
		}
		self
	}
}

impl<'a> ops::Sub for Array<'a> {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		for v in other.0 {
			if let Some(p) = self.0.iter().position(|x| *x == v) {
				self.0.remove(p);
			}
		}
		self
	}
}

// ------------------------------

pub trait Abolish<T> {
	fn abolish<F>(&mut self, f: F)
	where
		F: FnMut(usize) -> bool;
}

impl<T> Abolish<T> for Vec<T> {
	fn abolish<F>(&mut self, mut f: F)
	where
		F: FnMut(usize) -> bool,
	{
		let mut i = 0;
		// FIXME: use drain_filter once stabilized (https://github.com/rust-lang/rust/issues/43244)
		// to avoid negation of the predicate return value.
		self.retain(|_| {
			let retain = !f(i);
			i += 1;
			retain
		});
	}
}

// ------------------------------

pub(crate) trait Clump<T> {
	fn clump(self, clump_size: usize) -> T;
}

impl<'a> Clump<Array<'a>> for Array<'a> {
	fn clump(self, clump_size: usize) -> Array<'a> {
		self.0
			.chunks(clump_size)
			.map::<Cow<Value>, _>(|chunk| Cow::Owned(chunk.to_vec().into()))
			.collect::<Vec<_>>()
			.into()
	}
}

// ------------------------------

pub(crate) trait Combine<T> {
	fn combine(self, other: T) -> T;
}

impl<'a> Combine<Array<'a>> for Array<'a> {
	fn combine(self, other: Self) -> Array<'a> {
		let mut out = Self::with_capacity(self.len().saturating_mul(other.len()));
		for a in self.iter() {
			for b in other.iter() {
				out.push(vec![*a, *b].into());
			}
		}
		out
	}
}

// ------------------------------

pub(crate) trait Complement<T> {
	fn complement(self, other: T) -> T;
}

impl<'a> Complement<Array<'a>> for Array<'a> {
	fn complement(self, other: Self) -> Array<'a> {
		let mut out = Array::new();
		for v in self.into_iter() {
			if !other.contains(&v) {
				out.push(v)
			}
		}
		out
	}
}

// ------------------------------

pub(crate) trait Concat<T> {
	fn concat(self, other: T) -> T;
}

impl<'a> Concat<Array<'a>> for Array<'a> {
	fn concat(mut self, mut other: Array) -> Array {
		self.append(&mut other);
		self
	}
}

// ------------------------------

pub(crate) trait Difference<T> {
	fn difference(self, other: T) -> T;
}

impl<'a> Difference<Array<'a>> for Array<'a> {
	fn difference(self, mut other: Array) -> Array {
		let mut out = Array::new();
		for v in self.into_iter() {
			if let Some(pos) = other.iter().position(|w| v == *w) {
				other.remove(pos);
			} else {
				out.push(v);
			}
		}
		out.append(&mut other);
		out
	}
}

// ------------------------------

pub(crate) trait Flatten<T> {
	fn flatten(self) -> T;
}

impl<'a> Flatten<Array<'a>> for Array<'a> {
	fn flatten(self) -> Array<'a> {
		let mut out = Array::new();
		for v in self.into_iter() {
			match v {
				Cow::Borrowed(Value::Array(a)) => out.extend(a.iter().map(|c| *c)),
				Cow::Owned(Value::Array(mut a)) => out.append(&mut a),
				_ => out.push(v),
			}
		}
		out
	}
}

// ------------------------------

pub(crate) trait Intersect<T> {
	fn intersect(self, other: T) -> T;
}

impl<'a> Intersect<Self> for Array<'a> {
	fn intersect(self, mut other: Self) -> Self {
		let mut out = Self::new();
		for v in self.0.into_iter() {
			if let Some(pos) = other.iter().position(|w| v == *w) {
				other.remove(pos);
				out.push(v);
			}
		}
		out
	}
}

// ------------------------------

// Documented with the assumption that it is just for arrays.
pub(crate) trait Matches<C, T> {
	/// Returns an array complimenting the origional where each value is true or false
	/// depending on whether it is == to the compared value.
	///
	/// Admittedly, this is most often going to be used in `count(array::matches($arr, $val))`
	/// to count the number of times an element appears in an array but it's nice to have
	/// this in addition.
	fn matches(self, compare_val: C) -> T;
}

impl<'a> Matches<CowValue<'a>, Array<'a>> for Array<'a> {
	fn matches(self, compare_val: CowValue<'a> ) -> Array<'a> {
		self.iter().map(|arr_val| (*arr_val == compare_val).into()).collect::<Vec<Value>>().into()
	}
}
impl<'a> Matches<Value, Array<'a>> for Array<'a> {
	fn matches(self, compare_val: Value) -> Array<'a> {
		self.iter()
			.map(|arr_val| (arr_val.as_ref() == &compare_val).into())
			.collect::<Vec<Value>>()
			.into()
	}
}

// ------------------------------

// Documented with the assumption that it is just for arrays.
pub(crate) trait Transpose<T> {
	/// Stacks arrays on top of each other. This can serve as 2d array transposition.
	///
	/// The input array can contain regular values which are treated as arrays with
	/// a single element.
	///
	/// It's best to think of the function as creating a layered structure of the arrays
	/// rather than transposing them when the input is not a 2d array. See the examples
	/// for what happense when the input arrays are not all the same size.
	///
	/// Here's a diagram:
	/// [0, 1, 2, 3], [4, 5, 6]
	/// ->
	/// [0    | 1    | 2   |  3]
	/// [4    | 5    | 6   ]
	///  ^      ^      ^      ^
	/// [0, 4] [1, 5] [2, 6] [3]
	///
	/// # Examples
	///
	/// ```ignore
	/// fn array(sql: &str) -> Array {
	///     unimplemented!();
	/// }
	///
	/// // Example of `transpose` doing what it says on the tin.
	/// assert_eq!(array("[[0, 1], [2, 3]]").transpose(), array("[[0, 2], [1, 3]]"));
	/// // `transpose` can be thought of layering arrays on top of each other so when
	/// // one array runs out, it stops appearing in the output.
	/// assert_eq!(array("[[0, 1], [2]]").transpose(), array("[[0, 2], [1]]"));
	/// assert_eq!(array("[0, 1, 2]").transpose(), array("[[0, 1, 2]]"));
	/// ```
	fn transpose(self) -> T;
}

impl<'a> Transpose<Array<'a>> for Array<'a> {
	fn transpose(self) -> Array<'a> {
		if self.is_empty() {
			return self;
		}
		// I'm sure there's a way more efficient way to do this that I don't know about.
		// The new array will be at *least* this large so we can start there;
		let mut transposed_vec = Vec::<Value>::with_capacity(self.len());
		let mut iters = self
			.iter()
			.map(|v| {
				if let Value::Array(arr) = v {
					Box::new(arr.into_iter()) as Box<dyn ExactSizeIterator<Item = CowValue<'a>>>
				} else {
					Box::new(std::iter::once(v)) as Box<dyn ExactSizeIterator<Item = CowValue<'a>>>
				}
			})
			.collect::<Vec<_>>();
		// We know there is at least one element in the array therefore iters is not empty.
		// This is safe.
		let longest_length = iters.iter().map(|i| i.len()).max().unwrap();
		for _ in 0..longest_length {
			transposed_vec
				.push(iters.iter_mut().filter_map(|i| i.next()).collect::<Vec<_>>().into());
		}
		transposed_vec.into()
	}
}

// ------------------------------

pub(crate) trait Union<T> {
	fn union(self, other: T) -> T;
}

impl<'a> Union<Self> for Array<'a> {
	fn union(mut self, mut other: Self) -> Array<'a> {
		self.append(&mut other);
		self.uniq()
	}
}

// ------------------------------

pub(crate) trait Uniq<T> {
	fn uniq(self) -> T;
}

impl<'a> Uniq<Array<'a>> for Array<'a> {
	fn uniq(mut self) -> Array<'a> {
		let mut set: HashSet<&Value> = HashSet::new();
		let mut to_remove: Vec<usize> = Vec::new();
		for (i, item) in self.iter().enumerate() {
			if !set.insert(item) {
				to_remove.push(i);
			}
		}
		for i in to_remove.iter().rev() {
			self.remove(*i);
		}
		self
	}
}

// ------------------------------

pub fn array(i: &str) -> IResult<&str, Array> {
	let (i, v) =
		delimited_list0(openbracket, commas, terminated(value, mightbespace), char(']'))(i)?;
	Ok((i, Array(v.into_iter().map(Cow::Owned).collect())))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn array_empty() {
		let sql = "[]";
		let res = array(sql);
		let out = res.unwrap().1;
		assert_eq!("[]", format!("{}", out));
		assert_eq!(out.0.len(), 0);
	}

	#[test]
	fn array_normal() {
		let sql = "[1,2,3]";
		let res = array(sql);
		let out = res.unwrap().1;
		assert_eq!("[1, 2, 3]", format!("{}", out));
		assert_eq!(out.0.len(), 3);
	}

	#[test]
	fn array_commas() {
		let sql = "[1,2,3,]";
		let res = array(sql);
		let out = res.unwrap().1;
		assert_eq!("[1, 2, 3]", format!("{}", out));
		assert_eq!(out.0.len(), 3);
	}

	#[test]
	fn array_expression() {
		let sql = "[1,2,3+1]";
		let res = array(sql);
		let out = res.unwrap().1;
		assert_eq!("[1, 2, 3 + 1]", format!("{}", out));
		assert_eq!(out.0.len(), 3);
	}

	#[test]
	fn array_fnc_clump() {
		fn test(input_sql: &str, clump_size: usize, expected_result: &str) {
			let arr_result = array(input_sql);
			let arr = arr_result.unwrap().1;
			let clumped_arr = arr.clump(clump_size);
			assert_eq!(format!("{}", clumped_arr), expected_result);
		}

		test("[0, 1, 2, 3]", 2, "[[0, 1], [2, 3]]");
		test("[0, 1, 2, 3, 4, 5]", 3, "[[0, 1, 2], [3, 4, 5]]");
		test("[0, 1, 2]", 2, "[[0, 1], [2]]");
		test("[]", 2, "[]");
	}

	#[test]
	fn array_fnc_transpose() {
		fn test(input_sql: &str, expected_result: &str) {
			let arr_result = array(input_sql);
			let arr = arr_result.unwrap().1;
			let transposed_arr = arr.transpose();
			assert_eq!(format!("{}", transposed_arr), expected_result);
		}

		test("[[0, 1], [2, 3]]", "[[0, 2], [1, 3]]");
		test("[[0, 1], [2]]", "[[0, 2], [1]]");
		test("[[0, 1, 2], [true, false]]", "[[0, true], [1, false], [2]]");
		test("[[0, 1], [2, 3], [4, 5]]", "[[0, 2, 4], [1, 3, 5]]");
	}

	#[test]
	fn array_fnc_uniq_normal() {
		let sql = "[1,2,1,3,3,4]";
		let res = array(sql);
		let out = res.unwrap().1.uniq();
		assert_eq!("[1, 2, 3, 4]", format!("{}", out));
		assert_eq!(out.0.len(), 4);
	}
}
