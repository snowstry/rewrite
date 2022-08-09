use yew::prelude::*;

mod components;
use components::text::Hello;

fn main() {
    yew::start_app::<App>();
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Hello />
        </>
    }
}
