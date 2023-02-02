use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};
use stylist::{yew::styled_component, Style};

mod main_title;

use main_title::MainTitle;

#[derive(Serialize, Deserialize)]
struct Programer {
    name: String,
    age: i32,

    language: String,
}

const MAIN_CSS_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {

    let global_style = Style::new(MAIN_CSS_FILE).unwrap();

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

    let paragraph = html!{
        <p>{"My paragraph"}</p>
    };

    let vector_list = vec!["Alice", "Augusto", "Luffy"];

    return html! {

        <div class = {global_style}>
            <div id = {"container"}>
                <MainTitle title = "My title"/>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
                <p>{"Hi everyone"}</p>

                if *counter > 0 {
                    {paragraph}
                } else {
                    <p>{"Click the button"}</p>
                }

                <br/>

                <h2>{"My list"}</h2>

                <ul>
                    {str_list_to_html_list(vector_list)}
                </ul>

            </div>
        </div>
    };
}

fn str_list_to_html_list(list: Vec<&str>) -> Vec<Html> {
    return list.iter().map(|list_item| html!{<li>{list_item}</li>}).collect();
}