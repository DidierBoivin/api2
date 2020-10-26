extern crate actix_rt;
extern crate actix_web;
extern crate actix;

#[macro_use]
extern crate diesel;
use  diesel::pg::PgConnection;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
//extern crate diesel;
extern crate bigdecimal;
extern crate juniper_eager_loading;

#[macro_use] 
extern crate serde_derive;

//#[macro_use] 
extern crate serde_json;  
use futures::future;


pub mod models;
pub mod schema;
pub mod graphql;


use models::{MyappPathology,MyappBreakdowntype};
use std::{env, io};
//use std::sync::mpsc;

use actix_web::{get,middleware, App, HttpServer,HttpResponse};

mod db;
use db::get_pool;

mod endpoints;

use endpoints::graphql_endpoints;

mod error_handler;
use error_handler::CustomError;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    /*let (tx,rx) = mpsc::channel();
    thread::spawn(move||{
        let sys = System::new("loging");
        let srv = logging_setup();
        let _ = tx.send(srv);
        sys.run()

    });
    let srv = rx.recv().unwrap();
    */
    logging_setup();

    // Instantiate a new connection pool
    let pool = get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    let first = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)})
            .bind("0.0.0.0:4000")?
            .run();
    let second = HttpServer::new(move || 
        App::new()
            //.service(index)
            .service(all_pathologys)
            .service(all_breakdowns)
            //.service(all_cases)
            )
               //.service(all_agendas)
            .bind("0.0.0.0:5000")?
            .run();
    future::try_join(first,second).await ?;
    Ok(())    
}
/*
#[actix_rt::secondary]
async fn secondary() -> io::Result<()> {


    
    
}*/
// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
}
/*
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    connect();
    
    HttpServer::new(|| App::new()
            .service(index)
            .service(all_pathologys)
            //.service(all_breakdowns)
            //.service(all_cases)
            .service(all_agendas)
            
        )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}  
*/

#[get("/pathology")]
async fn all_pathologys() -> Result<HttpResponse,CustomError> {
    let connexion = get_pool().get().unwrap();
    let pathologys = MyappPathology::all (&connexion);
    Ok(HttpResponse::Ok().json(pathologys))
}
#[get("/breakdowntype")]
async fn all_breakdowns() -> Result<HttpResponse,CustomError> {
    let connexion = get_pool().get().unwrap();
    let breakdowntypes = MyappBreakdowntype::all (&connexion);
    Ok(HttpResponse::Ok().json(breakdowntypes))
}



/*#[get("/case")]
async fn all_cases() -> Result<HttpResponse,CustomError> {
    let connexion = get_pool().get().unwrap();
    let cases = MyappCase::all (&connexion);
    Ok(HttpResponse::Ok().json(cases))
}
*/