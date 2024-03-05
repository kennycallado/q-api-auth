#[derive(Debug)]
pub enum AuthError {
	MissingToken,
	InvalidToken,
}
