use crate::talk::entity::{TalkOptions, TalkProvider, TalkRoute};
// use ntex::http::HttpService;
// use ntex::server::Server;
// use std::io;
pub fn new(opt: TalkOptions, rt: &Vec<TalkRoute>) -> TalkProvider {
    return TalkProvider {};
}

// #[ntex::main]
// async fn serve() -> io::Result<()> {
//     Server::build()
//         .bind("echo", "127.0.0.1:8080", |_| {
//             HttpService::build().finish(handle_request)
//         })?
//         .run()
//         .await
// }
