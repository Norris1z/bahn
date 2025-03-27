use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::constants::{ASCII_CODE, IMAGE_CODE, NON_PRINT};
use crate::filesystem::file::representation_type::RepresentationType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

const SUPPORTED_REPRESENTATION_TYPES: &str =
    "Unsupported type. Supported types are I, I N, A and A N";

pub struct TypeCommandHandler<'a> {
    code: &'a CommandArgument<'a>,
    option: &'a CommandArgument<'a>,
}

impl<'a> TypeCommandHandler<'a> {
    pub fn new(code: &'a CommandArgument<'a>, option: &'a CommandArgument<'a>) -> Self {
        Self { code, option }
    }

    fn validate_argument(
        &self,
        argument: &str,
        valid_options: Vec<char>,
    ) -> (bool, Option<char>, ResponseCollection) {
        if argument.len() > 1 {
            return (
                false,
                None,
                vec![Response::new(
                    ResponseCode::SyntaxErrorInParametersOrArguments,
                    ResponseMessage::Custom(SUPPORTED_REPRESENTATION_TYPES),
                    ResponseType::Complete,
                )],
            );
        }

        let code_char = argument.chars().next().unwrap();

        if !valid_options.contains(&code_char) {
            return (
                false,
                None,
                vec![Response::new(
                    ResponseCode::SyntaxErrorInParametersOrArguments,
                    ResponseMessage::Custom(SUPPORTED_REPRESENTATION_TYPES),
                    ResponseType::Complete,
                )],
            );
        }

        (true, Some(code_char), vec![])
    }
}

impl<'a> CommandHandler for TypeCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let code = self.code.as_ref().unwrap().to_uppercase();

        let code_validation = self.validate_argument(&code, vec![ASCII_CODE, IMAGE_CODE]);

        if !code_validation.0 {
            return code_validation.2;
        }

        let mut option_char = None;

        if self.option.is_some() {
            let option = self.option.as_ref().unwrap().to_uppercase();
            let option_validation = self.validate_argument(&option, vec![NON_PRINT]);

            if !option_validation.0 {
                return option_validation.2;
            }

            option_char = option_validation.1;
        }

        context.set_representation_type(RepresentationType::from(
            code_validation.1.unwrap(),
            option_char,
        ));

        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::CustomString(format!(
                "Type set to {} {}",
                code_validation.1.unwrap(),
                option_char.unwrap_or(' ')
            )),
            ResponseType::Complete,
        )]
    }
}
