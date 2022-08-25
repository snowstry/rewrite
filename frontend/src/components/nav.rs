use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub links: HashMap<String, String>,
}

#[function_component(Nav)]
pub fn navbar(props: &Props) -> Html {
    let navbar_active = use_state_eq(|| false);
    let search_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let toggle_search = {
        let search_active = search_active.clone();

        Callback::from(move |_| {
            search_active.set(!*search_active);
        })
    };

    let nav_active_class = if !*navbar_active { "hidden" } else { "" };
    let search_active_class = if !*search_active { "hidden" } else { "" };

    let link_list: Html = props
        .links
        .iter()
        .map(|(key, value)| {
            html! {
                <a href={value.to_owned()} class="lg:inline-flex lg:w-auto w-full px-3 py-2 rounded-lg text-nord_light-300 items-center justify-center hover:bg-nord_dark-300">{key}</a>
            }
        })
        .collect::<Html>();

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
                <button class="bg-nord_dark-300 p-3 rounded-full mt-1 outline-none ease-in-out" onclick={&toggle_search}>{"S"}</button>

                // Modal
                <div class={classes!("fade", "flex", "justify-center", "items-center", "overflow-x-hidden", "overflow-y-auto", "fixed", "inset-0", "z-50", "outline-none", "ml-2", "mr-2", "font-jetbrains", search_active_class)}>
                    <div class="relative w-full my-6 mx-auto max-w-3xl drop-shadow-xl">
                        <div class="rounded-lg relative flex flex-col w-full bg-nord_dark-400 outline-none">
                            <div class="flex items-start justify-between">
                                <p class="text-nord_dark-100 mt-5 ml-5">{"Search users"}</p>
                                <button onclick={toggle_search}>
                                    <span class="block text-nord_dark-100 rounded-full text-xl mt-5 mr-5">{"Close"}</span>
                                </button>
                            </div>
                            <div class="relative p-6 flex-auto">
                                <input class="bg-nord_dark-200 outline-none rounded-lg pt-2 pb-2 pl-4 pr-4 w-full" placeholder="Search" />
                                <ul>
                                    <li class="bg-nord_dark-300 mt-2 p-2 rounded-lg">{"Duncus"}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>

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
                <div class={classes!("w-full", "lg:inline-flex", "lg:flex-grow", "text-center", "lg:w-auto", nav_active_class)}>
                    <div class="lg:inline-flex lg:flex-row lg:ml-auto lg:w-auto w-full lg:items-center items-start  flex flex-col lg:h-auto">
                        {link_list}
                        <a href="http://localhost:3001/login/google" class="lg:inline-flex lg:w-auto w-full px-3 py-2 rounded-lg text-white items-center justify-center hover:bg-nord_dark-300 text-nord_red">
                            {"Log out"}
                        </a>
                    </div>
                </div>
        </nav>
    }
}
