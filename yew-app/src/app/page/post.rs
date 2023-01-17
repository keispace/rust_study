use crate::app::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let mut next_id: i32 = props.id.parse::<i32>().unwrap() + 1;
    next_id = if next_id > 10 { 10 } else { next_id };

    let mut pre_id: i32 = props.id.parse::<i32>().unwrap() - 1 | 0;
    pre_id = if pre_id < 1 { 1 } else { pre_id };

    let go_post = |n: String| {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Post {
                id: n.to_string().to_owned(),
            })
        })
    };

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {  <main>
        <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
        <h1>{ "Hello Post !" }</h1>
        <h2>{format!("You are looking at Post {}", props.id)}</h2>
        <button onclick={go_post(pre_id.to_string().to_owned())}>{ "Go pre Post " }</button>
        <button onclick={go_home}>{ "Go Home" }</button>
        <button onclick={go_post(next_id.to_string().to_owned())}>{ "Go next Post " }</button>
    </main> }
}
