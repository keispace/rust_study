use crate::app::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let go_secure = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Secure))
    };
    let go_post = |n: String| {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Post {
                id: n.to_string().to_owned(),
            })
        })
    };
    html! {  <main>
        <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
        <h1>{ "Hello World!" }</h1>
        <button onclick={go_secure}>{ "Go Secure" }</button>
        <button onclick={go_post("1".to_owned())}>{ "Go post 1" }</button>
    </main> }
}
