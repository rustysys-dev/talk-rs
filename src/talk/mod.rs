use std::io;
mod adapter;
pub mod entity;
use crate::talk::entity::{TalkOptions, TalkProvider, TalkRoute, TalkType};

pub trait Talk {
    fn about(&self);
    fn serve(&self) -> io::Result<()>;
}

struct TalkInstance {
    name: String,
    providers: Vec<TalkProvider>,
}

impl Talk for TalkInstance {
    fn about(&self) {
        println!("{}", self.name);
    }
    fn serve(&self) -> io::Result<()> {
        // implment iterator over providers to serve multiple servers
        return Ok({});
    }
}

pub fn new(name: &str, opts: Vec<TalkOptions>, routes: Vec<TalkRoute>) -> impl Talk {
    let providers = opts
        .into_iter()
        .map(|opt| match opt.server_type {
            TalkType::HTTP => adapter::http::new(opt, &routes),
            TalkType::GRPC => adapter::grpc::new(opt, &routes),
            _ => panic!("bad talk type: {}", opt.server_type.as_ref()),
        })
        .collect();
    return TalkInstance {
        name: String::from(name),
        providers: providers,
    };
}
