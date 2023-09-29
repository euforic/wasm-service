use crate::{site::*, Handler};
use html_to_string_macro::html;
use std::sync::Mutex;

pub(crate) fn register(router: &mut matchit::Router<Handler>) -> Result<(), matchit::InsertError> {
    const TITLE: &str = "Hello WASM Service Worker";
    router.insert("/", |_, r| page(TITLE, about(r.path())))?;
    router.insert("/;nav", |_, r| nav(TITLE, about(r.path())))?;
    router.insert("/;clicked", |_, r| about_clicked(r.path()))?;
    Ok(())
}

static COUNTER: Mutex<u64> = Mutex::new(0);

fn about(url: &str) -> String {
    html! {
        { tabs(Tabs::About) }
        <h1>"HTMX + Service Workers + WebAssembly + Rust"</h1>
        <div>
            "This is a simple proof of concept that shows how you could use "
            <a href="https://htmx.org/">"HTMX"</a>" and Rust for frontend development.
        The basic idea in HTMX is that a webserver is being called on interactions
        with DOM elements, and returning back snippets of HTML that will replace
        certain DOM elements. Instead of going to a real web serviceWorker this
        project shows how service workers can intercept calls to a server and
        return back responses driven from WebAssembly instead."
            <br />
            "Repo: "<a href="https://github.com/euforic/wasm-service">"https://github.com/euforic/wasm-service"</a>
        </div>
        <br />
        <button hx-post="./;clicked" hx-swap="innerHTML" hx-target="#target">"Click Me"</button>
        <div>
            <div id="target">{ about_clicked_display(url) }</div>
        </div>
    }
}

fn about_clicked(url: &str) -> String {
    *(COUNTER.lock().unwrap()) += 1;

    about_clicked_display(url)
}

fn about_clicked_display(url: &str) -> String {
    html! {
        <div>
            "Hey <b>User</b>, this html is generated from Rust WASM using"
            " a service worker that intercepts http calls and returns HTML for "
            { url }
            <br />
            <p>"Clicked count here: " { *(COUNTER.lock().unwrap()) }</p>
        </div>
    }
}
