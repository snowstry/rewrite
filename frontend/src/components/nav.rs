use yew::prelude::*;

#[function_component(Nav)]
pub fn navbar() -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "hidden" } else { "" };

    html! {
        <nav class="flex items-center flex-wrap bg-nord_dark-400 px-2 text-nord_light-300">
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
                <button class=" inline-flex p-3 hover:bg-green-600 rounded lg:hidden text-white ml-auto hover:text-white outline-none" onclick={toggle_navbar}>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M4 6h16M4 12h16M4 18h16"
                        />
                    </svg>
                </button>
                <div class={classes!("w-full", "lg:inline-flex", "lg:flex-grow", "text-center", "lg:w-auto", active_class)}>
                    <div class="lg:inline-flex lg:flex-row lg:ml-auto lg:w-auto w-full lg:items-center items-start  flex flex-col lg:h-auto">
                        <a href="/" class="lg:inline-flex lg:w-auto w-full px-3 py-2 rounded-lg text-white items-center justify-center hover:bg-nord_dark-300 text-nord_red">
                            {"Log out"}
                        </a>
                    </div>
                </div>
        </nav>
    }
}
