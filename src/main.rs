mod talk;
mod usecase;
use std::env;
use talk::{entity::TalkOptions, entity::TalkRoute, entity::TalkType, Talk};
use usecase::login;

#[async_std::main]
async fn main() {
    env::set_var("RUST_LOG", "echo=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let t = talk::new(
        "my_server",
        vec![
            TalkOptions {
                server_type: TalkType::HTTP,
            },
            TalkOptions {
                server_type: TalkType::GRPC,
            },
        ],
        vec![TalkRoute {
            name: "login",
            func: &login,
        }],
    );
    t.about();
    t.serve().unwrap_or_else(|err| panic!("{:?}", err))
}
