use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
use pages::home::Home;
use pages::hello::Hello;
use pages::notfound::NotFound;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    HelloWorld,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    yew::start_app::<Main>();
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::HelloWorld => html! {
            <Hello />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
