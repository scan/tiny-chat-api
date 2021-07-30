mod filter;
mod graphql;
mod handler;
mod auth;
mod config;

use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if dotenv::dotenv().is_err() {
        log::warn!("loading environment variabled failed")
    };

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "tinychat=info");
    }

    env_logger::init();

    let cfg = config::Config::from_env();
    let auth_manager = auth::Manager::new(&cfg);

    let api = filter::all(auth_manager);

    let routes = api
      .with(warp::log("tinychat"))
      .with(
        warp::cors()
          .allow_any_origin()
          .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
          .allow_credentials(true)
          .allow_headers(vec![
            "Accept",
            "Authorization",
            "Content-Type",
            "X-CSRF-Token",
            "Accept-Language",
          ])
          .expose_header("Link")
          .max_age(300),
      )
      .with(warp::compression::gzip());
  
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
