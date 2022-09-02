use crate::components::nav::Nav;
use std::collections::HashMap;
use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut links: HashMap<String, String> = HashMap::new();
        links.insert("Home".to_string(), "/".to_string());
        links.insert("Profile".to_string(), "/profile".to_string());
        html! {
            <>
                <Nav links={links}/>
            </>
        }
    }
}
