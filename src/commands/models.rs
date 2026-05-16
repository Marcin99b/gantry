pub enum CommandType {
    Ping,
    GetTopics,
    CreateTopic,
    DeleteTopic,
    PutMessage,
    GetMessage,
    SubscribeTopic,
    UnsubscribeTopic,
}

impl TryFrom<u8> for CommandType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Ping),
            1 => Ok(CommandType::GetTopics),
            2 => Ok(CommandType::CreateTopic),
            3 => Ok(CommandType::DeleteTopic),
            4 => Ok(CommandType::PutMessage),
            5 => Ok(CommandType::GetMessage),
            6 => Ok(CommandType::SubscribeTopic),
            7 => Ok(CommandType::UnsubscribeTopic),
            v => Err(v),
        }
    }
}

pub struct Command {
    pub command_type: CommandType,
    pub data: Vec<u8>,
}
