mod api;
mod db;
mod error;
mod model;
mod prelude;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
// use surrealdb::opt::auth::Root;
use once_cell::sync::OnceCell;
use surrealdb::Surreal;

use api::*;

static DB: OnceCell<Surreal<Client>> =
    OnceCell::new();

const PORT: u16 = 8080;

#[actix_web::main]
async fn main(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("|> entrypoint");
    DB.set(Surreal::init()).expect("-1");
    DB.get()
        .expect(
            "failed to initialize surreal database",
        )
        .connect::<Ws>("localhost:8080")
        .await?;
    // println!("connected, signing in");
    // DB.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;
    // println!("signed in, setting ns/db");
    println!("|> setting ns/db");
    DB.get()
        .unwrap()
        .use_ns("test")
        .use_db("chef")
        .await?;

    println!("|> database connected");

    println!("|> server running at http://localhost:{PORT}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec![
                "GET", "POST", "PATCH", "DELETE",
            ])
            .send_wildcard();

        App::new()
            .wrap(cors)
            .service(create)
            .service(get)
            .service(update)
            .service(delete)
            .service(list)
    })
    .bind(("localhost", PORT))?
    .run()
    .await?;

    Ok(())
}
