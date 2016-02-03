pub mod vm;
pub mod gc;

#[derive(Debug)]
pub enum RuntimeError<'a> {
	LexerError {
		message: &a str
		line: usize
	}
}