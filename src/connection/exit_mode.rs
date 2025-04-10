use crate::constants::{CARRIAGE_RETURN_HEX, LINE_FEED_HEX, TELNET_IAC_HEX, TELNET_IP_HEX};

//not so sure about this, but this is mainly for telnet connections
#[derive(PartialEq)]
pub enum ExitMode {
    ControlMode,
    TelnetIACIPMode,
    None,
}
pub enum ControlFlowStatement {
    Continue(Option<ExitMode>),
    Break,
    TerminateAndBreak,
}

impl ExitMode {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let first_byte = bytes[0];

        /*
         * TODO: this currently only handles the IP of telnet. Should it handle all other codes?
         * https://www.ibm.com/docs/en/zos/2.4.0?topic=problems-telnet-commands-options#telcmds__rfc854
         */
        if bytes.len() > 2 && first_byte == TELNET_IAC_HEX && bytes[1] == TELNET_IP_HEX {
            Self::TelnetIACIPMode
        } else if first_byte.is_ascii_control() {
            Self::ControlMode
        } else {
            Self::None
        }
    }

    pub fn get_control_flow_statement(&self, bytes: &[u8]) -> Option<ControlFlowStatement> {
        //In case the user just sends a bunch of \r\n input, skip them
        if *self == ExitMode::None && bytes[0] == CARRIAGE_RETURN_HEX {
            return Some(ControlFlowStatement::Continue(None));
        }

        let exit_mode = ExitMode::from_bytes(&bytes);

        if exit_mode == ExitMode::ControlMode && bytes[bytes.len() - 1] == LINE_FEED_HEX {
            return Some(ControlFlowStatement::TerminateAndBreak);
        }

        if *self == ExitMode::None && exit_mode != ExitMode::None {
            return Some(ControlFlowStatement::Continue(Some(exit_mode)));
        }

        match self {
            ExitMode::ControlMode => Some(ControlFlowStatement::TerminateAndBreak),
            ExitMode::TelnetIACIPMode => Some(ControlFlowStatement::Break),
            ExitMode::None => None,
        }
    }
}
