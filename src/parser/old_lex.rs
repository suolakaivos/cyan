use ::runtime::RuntimeError;
use std::str::Bytes;

const KEYWORDS: [&'static str; 22] = [
	"and", "break", "do", "else", "elseif", "end",
	"false", "for", "function", "goto", "if", "in",
	"local", "nil", "not", "or", "repeat", "return",
	"then", "true", "until", "while"
];

#[inline]
pub fn is_iden(b: u8, is_first: bool) -> bool {
	!is_first && b >= b'0' && b <= b'9' ||
	b >= b'a' && b <= b'z' ||
	b >= b'A' && b <= b'Z' ||
	b == b'_'
}

#[inline]
pub fn is_space(b: u8) -> bool {
	b >= 9 && b <= 13 || b == 32
}

#[derive(Debug)]
pub enum Token<'a> {
	Identifier(&'a str),
	Keyword(&'a str),
	Comment(&'a str),

	String(Vec<u8>),

	Int(i64),
	Double(f64),

	Add,
	Sub,
	Mul,
	Pow,
	Div,
	Idiv,
	Mod,

	Band,
	Bor,
	Bnot, // Also Bxor
	Shl,
	Shr,
	Lt,
	Gt,
	Le,
	Ge,
	Concat,
	Dot,
	Varg,
	Neq,
	Eq,
	Len,

	Open,
	Close,

	SquareOpen,
	SquareClose,

	CurlyOpen,
	CurlyClose,

	Assign,

	Eof
}

#[derive(Debug)]
pub enum Reason {
	A,
	B,
	C
}

pub struct T<'a, 'b: 'a> {
	lexer: &'a Lexer<'b>,
	token: &'a Token<'b>,
	size: usize
}

pub struct Lexer<'a> {
	source: &'a str,
	offset: usize,
}

impl<'a> Lexer<'a> {
	#[inline]
	pub fn new(source: &'a str) -> Lexer<'a> {
		Lexer {
			source: source,
			offset: 0,
		}
	}

	#[inline]
	fn bytes(&self) -> Bytes<'a> {
		self.source[self.offset..].bytes()
	}

	#[inline]
	pub fn advance(&mut self, len: usize) {
		self.offset += len;
	}

	#[inline]
	pub fn read_token(&mut self) -> Result<(Token<'a>, usize), Reason> {
		let mut stream = self.bytes();

		match stream.next() {
			Some(b'\t' ... b'\r') | Some(b' ') => {
				self.offset += 1;
				self.read_token()
			},

			Some(b'(') => Ok((Token::Open, 1)),
			Some(b')') => Ok((Token::Close, 1)),
			Some(b'[') => Ok((Token::SquareOpen, 1)),
			Some(b']') => Ok((Token::SquareClose, 1)),
			Some(b'{') => Ok((Token::CurlyOpen, 1)),
			Some(b'}') => Ok((Token::CurlyClose, 1)),

			Some(b'+') => Ok((Token::Add, 1)),
			Some(b'*') => Ok((Token::Mul, 1)),
			Some(b'^') => Ok((Token::Pow, 1)),
			Some(b'%') => Ok((Token::Mod, 1)),
			Some(b'|') => Ok((Token::Bor, 1)),
			Some(b'#') => Ok((Token::Len, 1)),
			Some(b'&') => Ok((Token::Band, 1)),

			Some(b'.') => match stream.next() {
				Some(b'.') => match stream.next() {
					Some(b'.') => Ok((Token::Varg, 3)),
					_ => Ok((Token::Concat, 2))
				},

				_ => Ok((Token::Dot, 1))
			},

			Some(b'>') => match stream.next() {
				Some(b'>') => Ok((Token::Shr, 2)),
				Some(b'=') => Ok((Token::Ge, 2)),
				_ => Ok((Token::Gt, 1))
			},

			Some(b'<') => match stream.next() {
				Some(b'<') => Ok((Token::Shl, 2)),
				Some(b'=') => Ok((Token::Le, 2)),
				_ => Ok((Token::Lt, 1))
			},

			Some(b'~') => match stream.next() {
				Some(b'=') => Ok((Token::Neq, 2)),
				_ => Ok((Token::Bnot, 1))
			},

			Some(b'=') => match stream.next() {
				Some(b'=') => Ok((Token::Eq, 2)),
				_ => Ok((Token::Assign, 1))
			},

			Some(b'/') => match stream.next() {
				Some(b'/') => Ok((Token::Idiv, 2)),
				_ => Ok((Token::Div, 1))
			},

			Some(b'-') => match stream.next() {
				Some(b'-') => {
					let mut len = 2;

					loop {
						match stream.next() {
							Some(b'\n') | Some(b'\r') | None => break,
							_ => len += 1
						}
					}

					let comment = &self.source[self.offset..self.offset + len];
					Ok((Token::Comment(comment), len))
				},

				_ => Ok((Token::Sub, 1))
			},

			Some(end @ b'"') | Some(end @ b'\'') => {
				let mut data = Vec::new();
				let mut len = 0;

				loop {
					match stream.next() {
						Some(b'\\') => {
							match stream.next() {
								Some(b'a') => data.push(7),
								Some(b'b') => data.push(8),
								Some(b'f') => data.push(12),
								Some(b'n') => data.push(10),
								Some(b't') => data.push(9),
								Some(b'v') => data.push(11),
								None => return Err(Reason::A)
							}

							len += 2;
						},

						Some(b) => data.push(b),
					}
				}
			},

			Some(b) if is_iden(b, true) => {
				let mut len = 1;

				loop {
					match stream.next() {
						Some(b) if is_iden(b, false) => len += 1,
						_ => break
					}
				}

				let iden = &self.source[self.offset..self.offset + len];

				match KEYWORDS.contains(&iden) {
					true => Ok((Token::Keyword(iden), len)),
					false => Ok((Token::Identifier(iden), len))
				}
			},

			Some(end @ b'"') | Some(end @ b'\'') => {
				let mut data = Vec::new();
				let mut len = 1;

				loop {
					match stream.next() {
						Some(b) => {
							data.push(b);
							len += 1;
						},

						None => return Err(Reason::A)
					}
				}

				Ok((Token::String(data), len))
			},

			Some(_) => Err(Reason::A),

			None => Ok((Token::Eof, 0))
		}
	}
}