use html_to_string_macro::html;

pub(crate) fn page(title: &str, body: String) -> String {
    html! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <title>{ title }</title>
            <link rel="stylesheet" href="/assets/main.css" />
            <link rel="stylesheet" href="https://unpkg.com/water.css@2.1.1/out/water.css"
              integrity="sha384-eHoWBq4xGyEfS3rmZe6gvzlNS/nNJhiPPbKCJN1cQHJukU+q6ji3My2fJGYd1EBo"
              crossorigin="anonymous" />
            <script src="https://unpkg.com/htmx.org@1.8.2/dist/htmx.js"
              integrity="sha384-dUlt2hvoUDyqJ29JH9ln6o/B23lVQiQm8Z0+oEuPBWwKXiyG2MozxxFsCKWM7dLl"
              crossorigin="anonymous"></script>
            <script>r#"
                  if ("serviceWorker" in navigator) {
                    navigator.serviceWorker.register("/sw.js")
                      .then(reg => {
                        reg.addEventListener('statechange', event => {
                          console.log("received `statechange` event", { reg, event })
                        });
                        console.log("service worker registered", reg);
                        reg.active.postMessage({ type: 'clientattached' });
                      }).catch(err => {
                        console.error("service worker registration failed", err);
                      });
                    navigator.serviceWorker.addEventListener('controllerchange', event => {
                      console.log("received `controllerchange` event", event);
                    });
                  } else {
                    console.error("serviceWorker is missing from `navigator`. Note service workers must be served over https or on localhost");
                  }"#
            </script>
        </head>
        <body>
            { body }
        </body>
        </html>
    }
}

pub(crate) fn nav(title: &str, content: String) -> String {
    html! {
        <head><title>{ title }</title></head>
        { content }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum Tabs {
    About,
    Todos,
}

pub(crate) fn tabs(selected: Tabs) -> String {
    html! {
    <div class="nav-tabs" hx-target="closest body">
        <a href="" hx-get="./;nav" hx-push-url="" { if selected == Tabs::About { r#"class="selected""# } else {""} }>"About"</a>
        <a href="todos" hx-get="./todos;nav" hx-push-url="todos" { if selected == Tabs::Todos { r#"class="selected""# } else {""} }>"Todos"</a>
        <style>r#"
            .nav-tabs>a {
                padding: 1em;
                color: var(--text-bright);
            }
            .nav-tabs>a:hover {
                background-color: var(--button-hover) !important;
            }
            .nav-tabs>a.selected {
                background-color: var(--button-base);
            }
            "#
        </style>
    </div>
    }
}
