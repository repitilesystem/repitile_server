//! Contains the REST API server functionality.

use hyper::{Body, Response, StatusCode};

use gotham;
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::handler::{HandlerFuture, IntoHandlerError};

use futures::{future, Future, Stream};

use mime;

use repitile_core::{self, CommReq};

use std::thread;

/// Starts an instance of the REST API server in a new thread.
pub fn start() {
    let addr = "127.0.0.1:7878";

    thread::spawn(move || {
        gotham::start(addr, build_router());
    });
}

fn prof(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(body) => {
                let content = String::from_utf8(body.to_vec()).unwrap();

                println!("[DEBUG: Profile Req] Body = {}", content);

                (*super::GOTHAM_CHANNEL_SERVER_REQ)
                    .0
                    .send(CommReq::OpenProfile(content))
                    .unwrap();

                let status = match (*super::GOTHAM_CHANNEL_SERVER_RESP)
                    .1
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap()
                {
                    CommReq::Ok => {
                        println!("Loaded profile");
                        StatusCode::Ok
                    }
                    _ => {
                        println!("Failed to load profile");
                        StatusCode::NotFound
                    }
                };

                let res = create_response(
                    &state,
                    status,
                    Some((String::from("").into_bytes(), mime::TEXT_PLAIN)),
                );
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

fn config(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(body) => {
                let content = String::from_utf8(body.to_vec()).unwrap();

                println!("[DEBUG: Config Req] Body = {}", content);

                (*super::GOTHAM_CHANNEL_SERVER_REQ)
                    .0
                    .send(CommReq::OpenConfig(content))
                    .unwrap();

                let status = match (*super::GOTHAM_CHANNEL_SERVER_RESP)
                    .1
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap()
                {
                    CommReq::Ok => {
                        println!("Loaded config");
                        StatusCode::Ok
                    }
                    _ => {
                        println!("Failed to load config");
                        StatusCode::NotFound
                    }
                };

                let res = create_response(
                    &state,
                    status,
                    Some((String::from("").into_bytes(), mime::TEXT_PLAIN)),
                );
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

fn temp(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            repitile_core::CURRENT_CONDITIONS
                .lock()
                .unwrap()
                .temp
                .to_string()
                .into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, response)
}

fn humid(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            repitile_core::CURRENT_CONDITIONS
                .lock()
                .unwrap()
                .humidity
                .to_string()
                .into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, response)
}

fn build_router() -> Router {
    build_simple_router(|route| {
        route.scope("/get", |route| {
            route.get("/temp").to(temp);
            route.get("/humidity").to(humid);
        });

        route.scope("/set", |route| {
            route.post("/profile").to(prof);
            route.post("/config").to(config);
        });
    })
}
