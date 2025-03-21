pub enum ResponseMessage {
    Greeting,
    WrongCommand,
    CommandOkay,
    ProjectInfo,
    MissingArgument,
    Custom(&'static str),
}

impl ResponseMessage {
    pub fn get_message(&self) -> &str {
        match self {
            ResponseMessage::Greeting => "Welcome to the Bahn FTP Server v1.0.0", //refactor to make version dynamic
            ResponseMessage::WrongCommand => "Wrong command",
            ResponseMessage::CommandOkay => "Command okay",
            ResponseMessage::ProjectInfo => {
                "Please visit https://norris1z.com/bahn-ftp-server for more information"
            }
            ResponseMessage::MissingArgument => "Missing required argument",
            ResponseMessage::Custom(message) => message,
        }
    }
}
