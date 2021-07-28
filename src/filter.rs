use crate::auth;
use enclose::enclose;
use futures::FutureExt as _;
use std::{convert::Infallible, sync::Arc};
use warp::{header, Filter, Rejection, Reply};

use crate::{
    graphql::{schema, Context},
    handler,
};

use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{make_graphql_filter, subscriptions::serve_graphql_ws};

pub fn all(
    auth_manager: auth::Manager,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    health().or(graphql(auth_manager)).or(not_found())
}

fn not_found() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::any().and_then(handler::not_found)
}

fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(handler::health)
}

fn graphql(
    auth_manager: auth::Manager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let state = warp::any()
        .and(header::optional::<String>("authorization"))
        .map(enclose!((auth_manager) move |bearer_token| Context::new(auth_manager.clone(), bearer_token)));

    let root_node = Arc::new(schema());
    let graphql_filter = make_graphql_filter(schema(), state.boxed());

    let post_filter = warp::post()
        .and(warp::body::content_length_limit(1024 * 16))
        .and(graphql_filter.clone());

    let get_filter = warp::get().and(graphql_filter);

    let ws_filter = warp::ws()
        .and(header::optional::<String>("authorization"))
        .map(
            enclose!((auth_manager) move |ws: warp::ws::Ws, bearer_token: Option<String>| {
                ws.on_upgrade(
                    enclose!((auth_manager, root_node, bearer_token) move |websocket| async move {
                      serve_graphql_ws(
                        websocket,
                        root_node,
                        ConnectionConfig::new(Context::new(auth_manager, bearer_token)),
                      )
                      .map(|r| {
                        if let Err(e) = r {
                          println!("Websocket error: {}", e);
                        }
                      })
                      .await
                    }),
                )
            }),
        );

    warp::path("graphql").and(get_filter.or(post_filter).or(ws_filter))
}
