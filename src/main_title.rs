use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
   pub title: String
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {

   return html! {
      <h1>{&props.title}</h1>
   }
}