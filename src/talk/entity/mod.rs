use std::any::Any;
use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum TalkType {
    HTTP,
    GRPC,
}

pub struct TalkRoute {
    pub name: &'static str,
    pub func: &'static dyn Any,
}

pub struct TalkOptions {
    pub server_type: TalkType,
}

pub struct TalkProvider {}
