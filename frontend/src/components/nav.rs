use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props;

pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex items-center flex-wrap bg-nord_dark-400 px-2 text-nord_light-300">
            <nav>
                    <a href="/" class="inline-flex items-center p-2 mr-4 ">
                        <img
                            class="h-12 w-12 rounded-full"
                            src="https://raw.githubusercontent.com/snowstry/rewrite/main/frontend/public/default.png"
                            alt="Profile picture"
                            height={50}
                            width={50}
                        />
                        <p class="pl-4 pt-2">{"John Doe"}</p>
                    </a>
            </nav>
            </div>
        }
    }
}
