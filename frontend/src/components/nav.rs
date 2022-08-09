use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
}

pub struct Text;

impl Component for Text {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav className="flex items-center flex-wrap bg-nord_dark-400 pl-2 pr-2 text-nord_light-300 font-jetbrains">
            </nav>
        }
    }
}
