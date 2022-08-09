use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::text::Text;

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
            <Text text="Home" />
        },
        Route::HelloWorld => html! {
            <Text text="Hello world" />
        },
        Route::NotFound => html! {
            <Text text="404" />
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
