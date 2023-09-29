use crate::{site::*, Handler, Request};
use html_to_string_macro::html;
use serde::Deserialize;
use std::sync::{Mutex, RwLock};

pub(crate) fn register(router: &mut matchit::Router<Handler>) -> Result<(), matchit::InsertError> {
    const TODOS: &str = "Todos";
    router.insert("/todos", |_, _| page(TODOS, component()))?;
    router.insert("/todos;nav", |_, _| nav(TODOS, component()))?;
    router.insert("/todos;add", |_, r| hx_add(r))?;
    router.insert("/todos/:id", |p, _| hx_delete(p))?;
    router.insert("/todos/:id/toggle", |p, _| hx_toggle(p))?;
    router.insert("/todos;toggleall", |_, _| hx_toggleall())?;
    router.insert("/todos;filter=:filter", |p, _| hx_filter(p))?;
    Ok(())
}

struct Item {
    id: u32,
    done: bool,
    label: String,
}

#[derive(strum::EnumString, PartialEq, Clone, Copy)]
enum Filter {
    All,
    Active,
    Completed,
}

static TODO_ITEMS: RwLock<Vec<Item>> = RwLock::new(vec![]);
static TODO_INC: Mutex<u32> = Mutex::new(0);
static TODO_FILTER: Mutex<Filter> = Mutex::new(Filter::All);

fn component() -> String {
    html! {
        { tabs(Tabs::Todos) }
        <section class="todos">
            <header>
                <h1>"TODOs App"</h1>
                <div id="controls">
                { toggleall_frag(false) }
                { input_frag(false) }
                </div>
            </header>
            { items_frag(false) }
            <footer>
                { count_frag(false) }
                { filter_frag() }
            </footer>
            <style>r#"
                .todos {
                    font-size: 1.5em;
                }
                .todos input {
                    display: inline-block;
                    vertical-align: middle;
                }
                header input[type=checkbox] {
                    font-size: 1.5em;
                }
                .todos ul {
                    list-style-type: none;
                    padding-left: 0;
                }
                .todos li {
                    position: relative;
                    height: 2em;
                    width: 20em;
                    overflow: hidden;
                    padding-top: .2em;
                }
                .todos li>* {
                    vertical-align: middle;
                }
                .todos li>input[type=checkbox]:checked + label {
                    text-decoration: line-through;
                    color: var(--text-muted);
                }
                .todos button.delete:after {
                    content: '×';
                    font-size: 2em;
                    position: relative;
                    top: -.2em;
                }
                .todos button.delete {
                    position: absolute;
                    margin-top: -.2em;
                    right: 0;
                    padding: 0;
                    color: #af5b5e;
                    background-color: var(--background-body);
                }
                .todos button.delete:hover {
                    color: #ac262b;
                }

                "#
            </style>
        </section>
    }
}

fn items_frag(oob: bool) -> String {
    use Filter::*;
    let filter = *TODO_FILTER.lock().unwrap();
    let filter_fn = move |i: &'_ &Item| match filter {
        All => true,
        Active => !i.done,
        Completed => i.done,
    };
    html! {
        <ul id="items-list" { if oob { r#"hx-swap-oob="true""# } else { "" } }>
            { TODO_ITEMS.read().unwrap().iter().rev().filter(filter_fn).map(item_frag).collect::<String>() }
        </ul>
    }
}

fn item_frag(item: &Item) -> String {
    let id = item.id;
    html! {
        <li hx-target="this" hx-swap="outerHTML">
            <input type="checkbox" { if item.done { "checked" } else { "" }} hx-post={ format!("./todos/{id}/toggle") }/>
            <label>{ item.label.as_str() }</label>
            <button class="delete" hx-delete={ format!("./todos/{id}") }></button>
        </li>
    }
}

fn input_frag(oob: bool) -> String {
    html! {
        <input id="todo-new" name="todo-new" placeholder="What needs to be done?" autofocus
            hx-post="./todos;add" hx-target=".todos ul" hx-swap="afterbegin" { if oob { r#"hx-swap-oob="true""# } else { "" } } />
    }
}

fn toggleall_frag(oob: bool) -> String {
    let alldone = {
        let items = TODO_ITEMS.read().unwrap();
        items.len() > 0 && items.iter().all(|i| i.done)
    };

    html! {
        <input id="toggle-all" type="checkbox" { if alldone { "checked" } else { "" } }
            hx-post="./todos;toggleall" hx-target="this" { if oob { r#"hx-swap-oob="true""# } else { "" } } />
    }
}

fn count_frag(oob: bool) -> String {
    let len = TODO_ITEMS
        .read()
        .unwrap()
        .iter()
        .filter(|i| !i.done)
        .count();
    html! {
        <span id="todo-count" { if oob { r#"hx-swap-oob="true""# } else { "" } }><strong>{ len }</strong>" item" { if len == 1 { "" } else { "s" } } " left"</span>
    }
}

fn filter_frag() -> String {
    use Filter::*;
    let selected_filter = *(TODO_FILTER.lock().unwrap());
    html! {
    <fieldset class="filter" hx-swap="none">
        <legend>"Filter"</legend>
        <input type="radio" id="filter-all" name="filter" value="All" { if selected_filter == All {"checked"} else {""} }  hx-post="./todos;filter=All" />
        <label for="filter-all">"All"</label>
        <input type="radio" id="filter-active" name="filter" value="Active" { if selected_filter == Active {"checked"} else {""} } hx-post="./todos;filter=Active" />
        <label for="filter-active">"Active"</label>
        <input type="radio" id="filter-completed" name="filter" value="Completed" { if selected_filter == Completed {"checked"} else {""} } hx-post="./todos;filter=Completed" />
        <label for="filter-completed">"Completed"</label>
    </fieldset>
    <style>r#"
S        .filter {
            max-width: fit-content;
        }
        .filter input[type=radio] {
            display: none;
        }
        .filter label {
            margin-left: .5em;
            padding: .3em;
            min-width: 3em;
            text-align: center;
            border: .1em solid var(--text-muted);
            border-radius: .5em;
        }
        .filter label:hover {
            border-color: var(--highlight);
            color: var(--highlight);
            box-shadow: 0px 0px .1em .1em var(--highlight);
        }
        .filter input:checked + label {
            border-color: var(--text-bright);
            color: var(--text-bright);
            background-color: var(--button-base);
        }
        "#
    </style>
    }
}

fn hx_add(request: &Request) -> String {
    #[derive(Deserialize)]
    struct Form {
        #[serde(rename = "todo-new")]
        todo_new: String,
    }
    let label = match serde_urlencoded::from_str::<Form>(&request.body) {
        Err(e) => return html! { <p>"Error decoding form: " { e }</p> },
        Ok(value) => value.todo_new,
    };
    let id = {
        let mut inc = TODO_INC.lock().unwrap();
        *inc = inc.wrapping_add(1);
        *inc
    };
    TODO_ITEMS.write().unwrap().push(Item {
        id: id,
        done: false,
        label: label,
    });
    html! {
        { item_frag(TODO_ITEMS.read().unwrap().last().unwrap()) }
        { input_frag(true) }
        { count_frag(true) }
        { toggleall_frag(true) }
    }
}

fn hx_delete(params: &matchit::Params) -> String {
    let id = match params.get("id").map(str::parse::<u32>) {
        Some(Ok(id)) => id,
        Some(Err(_)) => return html! { <p>"Invalid param `id`"</p> },
        _ => return html! {<p>"Missing param `id`"</p> },
    };
    {
        let mut items = TODO_ITEMS.write().unwrap();
        if let Some(index) = items.iter().position(|i| i.id == id) {
            items.remove(index);
        }
    }
    html! {
        { count_frag(true) }
        { toggleall_frag(true) }
    }
}

fn hx_toggle(params: &matchit::Params) -> String {
    let id = match params.get("id").map(str::parse::<u32>) {
        Some(Ok(id)) => id,
        Some(Err(_)) => return html! { <p>"Invalid param `id`"</p> },
        _ => {
            return html! {<p>"Missing param `id`, got:" { params.iter().map(|(k,v)| format!("{k}={v}")).collect::<String>() }</p> }
        }
    };
    let (done, idx) = {
        match TODO_ITEMS
            .write()
            .unwrap()
            .iter_mut()
            .enumerate()
            .filter(|(_, i)| i.id == id)
            .next()
        {
            Some((idx, item)) => {
                item.done = !item.done;
                (item.done, idx)
            }
            _ => return html! { <p>"Invalid item number"</p> },
        }
    };
    use Filter::*;
    let filter = *(TODO_FILTER.lock().unwrap());
    html! {
        { match (done, filter) {
            (_, All) | (true, Completed) | (false, Active) => item_frag(&TODO_ITEMS.read().unwrap()[idx]),
            _ => "".to_string(),
        } }
        { count_frag(true) }
        { toggleall_frag(true) }
    }
}

fn hx_toggleall() -> String {
    let set = !TODO_ITEMS.read().unwrap().iter().all(|i| i.done);
    let mut dirty = false;
    for mut item in TODO_ITEMS.write().unwrap().iter_mut() {
        dirty = dirty || item.done != set;
        item.done = set;
    }
    html! {
        { count_frag(true) }
        { toggleall_frag(true) }
        { if dirty { items_frag(true) } else { "".to_string() } }
    }
}

fn hx_filter(params: &matchit::Params) -> String {
    let filter = match params.get("filter").map(str::parse::<Filter>) {
        Some(Ok(f)) => f,
        Some(Err(_)) => return html! { <p>"Invalid param `id`"</p> },
        _ => return html! {<p>"Missing param `id`"</p> },
    };
    *(TODO_FILTER.lock().unwrap()) = filter;
    html! {
        { filter_frag() }
        { items_frag(true) }
    }
}
