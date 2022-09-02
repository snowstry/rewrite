use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct Icon {}

impl Component for Icon {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <i class={format!("iconoir-{}", ctx.props().name)}></i>
        }
    }
}
