use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Hello;

impl Component for Hello {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Hello
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1 class="grid place-items-center h-screen text-nord_light-300">{"Hello world"}</h1>
        }
    }
}
