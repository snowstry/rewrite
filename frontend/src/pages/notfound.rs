use yew::prelude::*;

pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="grid place-items-center h-screen">
                <h1 class="text-nord_light-300">{"404 | Not found"}</h1>
            </div>
        }
    }
}
