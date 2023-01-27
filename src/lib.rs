use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Programer {
    name: String,
    age: i32,

    language: String,
}

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();

        let my_obj = Programer {
            name: "Matheus".to_owned(),
            age: 19,
            language: "Rust".to_owned()
        };

        log!(*counter);
        log!(serde_json::to_string_pretty(&my_obj).unwrap());

        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{"Hello World"}</h1>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}