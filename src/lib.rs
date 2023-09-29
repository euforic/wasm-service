use html_to_string_macro::html;
use matchit::{Params, Router};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

mod about;
mod site;
mod todos;
// Routing: https://docs.rs/matchit/latest/matchit/
// html! macro: https://docs.rs/html-to-string-macro/latest/html_to_string_macro/macro.html.html

type Handler = fn(&Params, &Request) -> String;

static ROUTER: Lazy<Result<Router<Handler>, matchit::InsertError>> = Lazy::new(|| {
    let mut router: Router<Handler> = Router::new();

    about::register(&mut router)?;
    todos::register(&mut router)?;

    Ok(router)
});

fn handle_request(request: &Request) -> String {
    let router = match ROUTER.as_ref() {
        Ok(r) => r,
        Err(e) => return html! { <p>"Failed to build router"</p><p>{ e }</p> },
    };
    let path = request.path().trim_start_matches("/wasm-service");
    let (handler, params) = match router.at(path) {
        Ok(ok) => (ok.value, ok.params),
        Err(matchit::MatchError::NotFound) => return html! { <p>"Not found"</p> },
        Err(e) => return html! { <p>"Error matching request handler: " {e}</p> },
    };
    handler(&params, request)
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(with = "http_serde::method")]
    method: http::method::Method,
    #[serde(with = "http_serde::uri")]
    url: http::Uri,
    headers: Vec<(String, String)>,
    body: String,
}

impl Request {
    fn path(&self) -> &str {
        self.url.path_and_query().unwrap().as_str()
    }
}

struct RoutingState {
    request: Option<Vec<u8>>,
    response: Option<String>,
}

static ROUTING_STATE: Mutex<RoutingState> = Mutex::new(RoutingState {
    request: None,
    response: None,
});

fn get_routing_state() -> MutexGuard<'static, RoutingState> {
    ROUTING_STATE.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn allocate_request(size: usize) -> *mut u8 {
    let mut rs = get_routing_state();
    rs.request = Some(vec![0; size]);
    rs.request.as_mut().unwrap().as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn fetch() -> usize {
    let mut rs = get_routing_state();
    let request_string = if let Some(ref request) = rs.request {
        String::from_utf8(request.clone()).unwrap()
    } else {
        String::from("{}")
    };
    rs.response = match serde_json::from_str(&request_string) {
        Ok(request) => Some(handle_request(&request)),
        Err(_) => Some("Failed to parse request string from service worker js".to_string()),
    };
    0
}

#[no_mangle]
pub extern "C" fn response_ptr() -> *const u8 {
    let rs = get_routing_state();

    if let Some(r) = &rs.response {
        r.as_ptr()
    } else {
        0 as *const u8
    }
}

#[no_mangle]
pub extern "C" fn response_len() -> usize {
    let rs = get_routing_state();

    if let Some(r) = &rs.response {
        r.len()
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn stop() -> usize {
    0
}
