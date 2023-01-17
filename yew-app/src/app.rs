use yew::prelude::*;
use yew_router::prelude::*;

mod route;
use route::Route;

mod page;
use page::home::Home;
use page::post::Post;
use page::secure::Secure;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Secure => html! { <Secure /> },
        Route::Post { id } => html! {<Post {id} />},
        _ => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render= {switch} />
        </BrowserRouter>
    }
}
