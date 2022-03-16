mod ws;
mod lobby;
use lobby::Lobby;
mod messages;
mod start_connection;
use start_connection::{start_connection as start_connection_route, send_statistics};
use actix::Actor;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route)
            .service(send_statistics) //register our route. rename with "as" import or naming conflict
            .data(chat_server.clone()) //register the lobby
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}