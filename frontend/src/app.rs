use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1 class="grid place-items-center h-screen text-nord_light-300 rounded-lg font-jetbrains">{ "Hello World!" }</h1>
        </main>
    }
}
