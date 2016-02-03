use ::parser::unicode::Lu;

#[derive(Debug)]
pub enum Token<'a> {
	Ident(&'a str, usize, usize),
}

pub struct Lex<'a> {
	source: &'a str
}