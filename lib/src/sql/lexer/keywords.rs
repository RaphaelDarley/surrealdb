use crate::sql::token::{Keyword, TokenKind};
use phf::phf_map;
use unicase::UniCase;

pub(crate) static KEYWORDS: phf::Map<UniCase<&'static str>, TokenKind> = phf_map! {
	UniCase::ascii("AFTER") => TokenKind::Keyword(Keyword::After),
	UniCase::ascii("ALL") => TokenKind::Keyword(Keyword::All),
	UniCase::ascii("ANALYZE") => TokenKind::Keyword(Keyword::Analyze),
	UniCase::ascii("ARABIC") => TokenKind::Keyword(Keyword::Arabic),
	UniCase::ascii("AS") => TokenKind::Keyword(Keyword::As),
	UniCase::ascii("ASCII") => TokenKind::Keyword(Keyword::Ascii),
	UniCase::ascii("ASSERT") => TokenKind::Keyword(Keyword::Assert),
	UniCase::ascii("BEFORE") => TokenKind::Keyword(Keyword::Before),
	UniCase::ascii("BEGIN") => TokenKind::Keyword(Keyword::Begin),
	UniCase::ascii("BLANK") => TokenKind::Keyword(Keyword::Blank),
	UniCase::ascii("BM25") => TokenKind::Keyword(Keyword::Bm25),
	UniCase::ascii("BREAK") => TokenKind::Keyword(Keyword::Break),
	UniCase::ascii("BY") => TokenKind::Keyword(Keyword::By),
	UniCase::ascii("CAMEL") => TokenKind::Keyword(Keyword::Camel),
	UniCase::ascii("CANCEL") => TokenKind::Keyword(Keyword::Cancel),
	UniCase::ascii("CHANGEFEED") => TokenKind::Keyword(Keyword::ChangeFeed),
	UniCase::ascii("CHANGES") => TokenKind::Keyword(Keyword::Changes),
	UniCase::ascii("CLASS") => TokenKind::Keyword(Keyword::Class),
	UniCase::ascii("COMMENT") => TokenKind::Keyword(Keyword::Comment),
	UniCase::ascii("COMMIT") => TokenKind::Keyword(Keyword::Commit),
	UniCase::ascii("CONTENT") => TokenKind::Keyword(Keyword::Content),
	UniCase::ascii("CONTINUE") => TokenKind::Keyword(Keyword::Continue),
	UniCase::ascii("COSINE") => TokenKind::Keyword(Keyword::Cosine),
	UniCase::ascii("CREATE") => TokenKind::Keyword(Keyword::Create),
	UniCase::ascii("DANISH") => TokenKind::Keyword(Keyword::Danish),
	UniCase::ascii("DATABASE") => TokenKind::Keyword(Keyword::Database),
	UniCase::ascii("DEFAULT") => TokenKind::Keyword(Keyword::Default),
	UniCase::ascii("DEFINE") => TokenKind::Keyword(Keyword::Define),
	UniCase::ascii("DELETE") => TokenKind::Keyword(Keyword::Delete),
	UniCase::ascii("DIFF") => TokenKind::Keyword(Keyword::Diff),
	UniCase::ascii("DROP") => TokenKind::Keyword(Keyword::Drop),
	UniCase::ascii("DUTCH") => TokenKind::Keyword(Keyword::Dutch),
	UniCase::ascii("EDDSA") => TokenKind::Keyword(Keyword::EdDSA),
	UniCase::ascii("EDGENGRAM") => TokenKind::Keyword(Keyword::Edgengram),
	UniCase::ascii("ENGLISH") => TokenKind::Keyword(Keyword::English),
	UniCase::ascii("ES256") => TokenKind::Keyword(Keyword::Es256),
	UniCase::ascii("ES384") => TokenKind::Keyword(Keyword::Es384),
	UniCase::ascii("ES512") => TokenKind::Keyword(Keyword::Es512),
	UniCase::ascii("EUCLIDEAN") => TokenKind::Keyword(Keyword::Euclidean),
	UniCase::ascii("EVENT") => TokenKind::Keyword(Keyword::Event),
	UniCase::ascii("EXPLAIN") => TokenKind::Keyword(Keyword::Explain),
	UniCase::ascii("FETCH") => TokenKind::Keyword(Keyword::Fetch),
	UniCase::ascii("FIELDS") => TokenKind::Keyword(Keyword::Fields),
	UniCase::ascii("FILTERS") => TokenKind::Keyword(Keyword::Filters),
	UniCase::ascii("FLEXIBILE") => TokenKind::Keyword(Keyword::Flexibile),
	UniCase::ascii("FOR") => TokenKind::Keyword(Keyword::For),
	UniCase::ascii("FRENCH") => TokenKind::Keyword(Keyword::French),
	UniCase::ascii("FROM") => TokenKind::Keyword(Keyword::From),
	UniCase::ascii("FULL") => TokenKind::Keyword(Keyword::Full),
	UniCase::ascii("FUNCTION") => TokenKind::Keyword(Keyword::Function),
	UniCase::ascii("GERMAN") => TokenKind::Keyword(Keyword::German),
	UniCase::ascii("GREEK") => TokenKind::Keyword(Keyword::Greek),
	UniCase::ascii("GROUP") => TokenKind::Keyword(Keyword::Group),
	UniCase::ascii("HAMMING") => TokenKind::Keyword(Keyword::Hamming),
	UniCase::ascii("HS256") => TokenKind::Keyword(Keyword::Hs256),
	UniCase::ascii("HS384") => TokenKind::Keyword(Keyword::Hs384),
	UniCase::ascii("HS512") => TokenKind::Keyword(Keyword::Hs512),
	UniCase::ascii("HUNGARIAN") => TokenKind::Keyword(Keyword::Hungarian),
	UniCase::ascii("IGNORE") => TokenKind::Keyword(Keyword::Ignore),
	UniCase::ascii("INDEX") => TokenKind::Keyword(Keyword::Index),
	UniCase::ascii("INFO") => TokenKind::Keyword(Keyword::Info),
	UniCase::ascii("INSERT") => TokenKind::Keyword(Keyword::Insert),
	UniCase::ascii("INTO") => TokenKind::Keyword(Keyword::Into),
	UniCase::ascii("IF") => TokenKind::Keyword(Keyword::If),
	UniCase::ascii("ITALIAN") => TokenKind::Keyword(Keyword::Italian),
	UniCase::ascii("IS") => TokenKind::Keyword(Keyword::Is),
	UniCase::ascii("KILL") => TokenKind::Keyword(Keyword::Kill),
	UniCase::ascii("LET") => TokenKind::Keyword(Keyword::Let),
	UniCase::ascii("LIMIT") => TokenKind::Keyword(Keyword::Limit),
	UniCase::ascii("LIVE") => TokenKind::Keyword(Keyword::Live),
	UniCase::ascii("LOWERCASE") => TokenKind::Keyword(Keyword::Lowercase),
	UniCase::ascii("MAHALANOBIS") => TokenKind::Keyword(Keyword::Mahalanobis),
	UniCase::ascii("MANHATTAN") => TokenKind::Keyword(Keyword::Manhattan),
	UniCase::ascii("MERGE") => TokenKind::Keyword(Keyword::Merge),
	UniCase::ascii("MINKOWSKI") => TokenKind::Keyword(Keyword::Minkowski),
	UniCase::ascii("MODEL") => TokenKind::Keyword(Keyword::Model),
	UniCase::ascii("NAMESPACE") => TokenKind::Keyword(Keyword::Namespace),
	UniCase::ascii("NGRAM") => TokenKind::Keyword(Keyword::Ngram),
	UniCase::ascii("NOINDEX") => TokenKind::Keyword(Keyword::NoIndex),
	UniCase::ascii("NONE") => TokenKind::Keyword(Keyword::None),
	UniCase::ascii("NOT") => TokenKind::Keyword(Keyword::Not),
	UniCase::ascii("NORWEGIAN") => TokenKind::Keyword(Keyword::Norwegian),
	UniCase::ascii("NULL") => TokenKind::Keyword(Keyword::Null),
	UniCase::ascii("OMIT") => TokenKind::Keyword(Keyword::Omit),
	UniCase::ascii("ON") => TokenKind::Keyword(Keyword::On),
	UniCase::ascii("ONLY") => TokenKind::Keyword(Keyword::Only),
	UniCase::ascii("OPTION") => TokenKind::Keyword(Keyword::Option),
	UniCase::ascii("ORDER") => TokenKind::Keyword(Keyword::Order),
	UniCase::ascii("PARALLEL") => TokenKind::Keyword(Keyword::Parallel),
	UniCase::ascii("PARAM") => TokenKind::Keyword(Keyword::Param),
	UniCase::ascii("PASSHASH") => TokenKind::Keyword(Keyword::Passhash),
	UniCase::ascii("PATCH") => TokenKind::Keyword(Keyword::Patch),
	UniCase::ascii("PERMISSIONS") => TokenKind::Keyword(Keyword::Permissions),
	UniCase::ascii("PORTUGUESE") => TokenKind::Keyword(Keyword::Portuguese),
	UniCase::ascii("PS256") => TokenKind::Keyword(Keyword::Ps256),
	UniCase::ascii("PS384") => TokenKind::Keyword(Keyword::Ps384),
	UniCase::ascii("PS512") => TokenKind::Keyword(Keyword::Ps512),
	UniCase::ascii("PUNCT") => TokenKind::Keyword(Keyword::Punct),
	UniCase::ascii("RELATE") => TokenKind::Keyword(Keyword::Relate),
	UniCase::ascii("REMOVE") => TokenKind::Keyword(Keyword::Remove),
	UniCase::ascii("REPLACE") => TokenKind::Keyword(Keyword::Replace),
	UniCase::ascii("RETURN") => TokenKind::Keyword(Keyword::Return),
	UniCase::ascii("ROLES") => TokenKind::Keyword(Keyword::Roles),
	UniCase::ascii("ROMANIAN") => TokenKind::Keyword(Keyword::Romanian),
	UniCase::ascii("ROOT") => TokenKind::Keyword(Keyword::Root),
	UniCase::ascii("RS256") => TokenKind::Keyword(Keyword::Rs256),
	UniCase::ascii("RS384") => TokenKind::Keyword(Keyword::Rs384),
	UniCase::ascii("RS512") => TokenKind::Keyword(Keyword::Rs512),
	UniCase::ascii("RUSSIAN") => TokenKind::Keyword(Keyword::Russian),
	UniCase::ascii("SCHEMAFULL") => TokenKind::Keyword(Keyword::Schemafull),
	UniCase::ascii("SCHEMALESS") => TokenKind::Keyword(Keyword::Schemaless),
	UniCase::ascii("SCOPE") => TokenKind::Keyword(Keyword::Scope),
	UniCase::ascii("SELECT") => TokenKind::Keyword(Keyword::Select),
	UniCase::ascii("SESSION") => TokenKind::Keyword(Keyword::Session),
	UniCase::ascii("SHOW") => TokenKind::Keyword(Keyword::Show),
	UniCase::ascii("SIGNIM") => TokenKind::Keyword(Keyword::Signim),
	UniCase::ascii("SIGNUP") => TokenKind::Keyword(Keyword::Signup),
	UniCase::ascii("SINCE") => TokenKind::Keyword(Keyword::Since),
	UniCase::ascii("SLEEP") => TokenKind::Keyword(Keyword::Sleep),
	UniCase::ascii("SNOWBALL") => TokenKind::Keyword(Keyword::Snowball),
	UniCase::ascii("SPANISH") => TokenKind::Keyword(Keyword::Spanish),
	UniCase::ascii("SPLIT") => TokenKind::Keyword(Keyword::Split),
	UniCase::ascii("START") => TokenKind::Keyword(Keyword::Start),
	UniCase::ascii("SWEDISH") => TokenKind::Keyword(Keyword::Swedish),
	UniCase::ascii("TABLE") => TokenKind::Keyword(Keyword::Table),
	UniCase::ascii("TAMIL") => TokenKind::Keyword(Keyword::Tamil),
	UniCase::ascii("THEN") => TokenKind::Keyword(Keyword::Then),
	UniCase::ascii("THROW") => TokenKind::Keyword(Keyword::Throw),
	UniCase::ascii("TIMEOUT") => TokenKind::Keyword(Keyword::Timeout),
	UniCase::ascii("TOKEIZERS") => TokenKind::Keyword(Keyword::Tokeizers),
	UniCase::ascii("TRANSACTION") => TokenKind::Keyword(Keyword::Transaction),
	UniCase::ascii("TURKISH") => TokenKind::Keyword(Keyword::Turkish),
	UniCase::ascii("TYPE") => TokenKind::Keyword(Keyword::Type),
	UniCase::ascii("UNIQUE") => TokenKind::Keyword(Keyword::Unique),
	UniCase::ascii("UPDATE") => TokenKind::Keyword(Keyword::Update),
	UniCase::ascii("UPPERCASE") => TokenKind::Keyword(Keyword::Uppercase),
	UniCase::ascii("USE") => TokenKind::Keyword(Keyword::Use),
	UniCase::ascii("USER") => TokenKind::Keyword(Keyword::User),
	UniCase::ascii("VALUE") => TokenKind::Keyword(Keyword::Value),
	UniCase::ascii("VERSION") => TokenKind::Keyword(Keyword::Version),
	UniCase::ascii("VS") => TokenKind::Keyword(Keyword::Vs),
	UniCase::ascii("WHEN") => TokenKind::Keyword(Keyword::When),
	UniCase::ascii("WHERE") => TokenKind::Keyword(Keyword::Where),
	UniCase::ascii("WITH") => TokenKind::Keyword(Keyword::With),
	UniCase::ascii("ANDKW") => TokenKind::Keyword(Keyword::AndKw),
	UniCase::ascii("AND") => TokenKind::Keyword(Keyword::And),
	UniCase::ascii("CONTAINSALL") => TokenKind::Keyword(Keyword::ContainsAll),
	UniCase::ascii("CONTAINSANY") => TokenKind::Keyword(Keyword::ContainsAny),
	UniCase::ascii("CONTAINSNONE") => TokenKind::Keyword(Keyword::ContainsNone),
	UniCase::ascii("CONTAINSNOT") => TokenKind::Keyword(Keyword::ContainsNot),
	UniCase::ascii("CONTAINS") => TokenKind::Keyword(Keyword::Contains),
	UniCase::ascii("ALLINSIDE") => TokenKind::Keyword(Keyword::AllInside),
	UniCase::ascii("ANYINSIDE") => TokenKind::Keyword(Keyword::AnyInside),
	UniCase::ascii("NONEINSIDE") => TokenKind::Keyword(Keyword::NoneInside),
	UniCase::ascii("NOTINSIDE") => TokenKind::Keyword(Keyword::NotInside),
	UniCase::ascii("INSIDE") => TokenKind::Keyword(Keyword::Inside),
	UniCase::ascii("INTERSECTS") => TokenKind::Keyword(Keyword::Intersects),
	UniCase::ascii("IN") => TokenKind::Keyword(Keyword::In),
	UniCase::ascii("OUTSIDE") => TokenKind::Keyword(Keyword::Outside),
	UniCase::ascii("OR") => TokenKind::Keyword(Keyword::OrKw),
};
