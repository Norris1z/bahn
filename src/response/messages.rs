use std::borrow::Cow;

pub enum ResponseMessage {
    Help,
    Quit,
    Greeting,
    WrongCommand,
    ProjectInfo,
    MissingArgument,
    Custom(&'static str),
    CustomString(String),
    UserNameOkay,
    LoginSuccessful,
    DirectoryNameCommentary(String, &'static str),
}

impl ResponseMessage {
    pub fn get_message(&self) -> Cow<'static, str> {
        match self {
            ResponseMessage::Greeting => Cow::Borrowed("Welcome to the Bahn FTP Server v1.0.0"), //refactor to make version dynamic
            ResponseMessage::WrongCommand => Cow::Borrowed("Wrong command"),
            ResponseMessage::ProjectInfo => Cow::Borrowed(
                "Please visit https://norris1z.com/bahn-ftp-server for more information",
            ),
            ResponseMessage::MissingArgument => Cow::Borrowed("Missing required argument"),
            ResponseMessage::Custom(message) => Cow::Borrowed(message),
            ResponseMessage::Help => Cow::Borrowed("Help Okay"),
            ResponseMessage::Quit => Cow::Borrowed("Goodbye"),
            ResponseMessage::UserNameOkay => Cow::Borrowed("Username okay, password needed"),
            ResponseMessage::LoginSuccessful => Cow::Borrowed("Login successful"),
            ResponseMessage::DirectoryNameCommentary(path, commentary) => {
                Cow::Owned(format!(r#""{}" {}"#, path, commentary))
            }
            ResponseMessage::CustomString(message) => Cow::Owned(message.to_owned()),
        }
    }
}
