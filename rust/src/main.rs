mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Serialize};

    #[derive(Serialize)]
    pub struct Good {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub price: i32,
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_postgres::{Row};

    use crate::models::Good;

    pub async fn select_goods(client: &Client) -> Vec<Good> {
        let _stmt = "SELECT id, name, description, price FROM goods";
        let stmt = client.prepare_cached(&_stmt).await.unwrap();

        client
            .query(&stmt, &[])
            .await
            .unwrap()
            .iter()
            .map(map_row)
            .collect::<Vec<Good>>()
    }

    fn map_row(row: &Row) -> Good {
        Good {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
        }
    }
}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};
    use crate::db;

    pub async fn get_goods(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let goods = db::select_goods(&client).await;
        Ok(HttpResponse::Ok().json(goods))
    }
}


use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::get_goods;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();


    let server = HttpServer::new(move || {
        App::new()
            // extra middleware not used, performance impact!
            //.wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/goods").route(web::get().to(get_goods)))
    })
    //.keep_alive(KeepAlive::Os)
    //.client_request_timeout(Duration::from_secs(0))
    //.backlog(1024)
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
