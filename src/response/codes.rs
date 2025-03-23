#[derive(Copy, Clone)]
pub enum ResponseCode {
    Success = 200,
    Quit = 221,
    Help = 214,
    SyntaxError = 500,
    SyntaxErrorInParametersOrArguments = 501,
    BadSequence = 503,
    UserNameOkay = 331,
    NotLoggedIn = 530,
    LoginSuccessful = 230,
    DirectoryName = 257,
}
