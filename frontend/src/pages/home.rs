use crate::components::nav::Nav;
use crate::components::text::Text;
use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Nav />
                <Text text="Home" />
            </>
        }
    }
}
