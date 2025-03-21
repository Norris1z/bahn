#[derive(Copy, Clone)]
pub enum ResponseCode {
    Success = 200,
    Help = 214,
    SyntaxError = 500,
    MissingArgument = 501,
    BadSequence = 503,
}
