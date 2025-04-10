#[derive(Copy, Clone, PartialEq)]
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
    RequestedActionNotTaken = 550,
    FileActionOkay = 250,
    CantOpenDataConnection = 425,
    EnteringPassiveMode = 227,
    StartingDataTransfer = 150,
    ClosingDataConnection = 226,
    ConnectionClosedTransferAborted = 426,
    SystemType = 215,
    ServiceReadyForNewUser = 220
}
