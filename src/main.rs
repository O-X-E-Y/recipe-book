#![feature(path_file_prefix)]

mod measurements;
mod recipe;
mod recipe_util;

use leptos_query::provide_query_client;
use recipe::*;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::{use_mouse, UseMouseReturn};
use stylance::import_style;

import_style!(pub css, "../styles/main.module.css");

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)

    // let recipes = RECIPES.iter().collect::<Vec<_>>();

    // dbg!(&recipes);

    // let s = include_str!("../recipes/boscaiola.txt");
    // let recipe = s.parse::<recipe_util::Recipe>();
    // println!("{recipe:?}")
}

#[component]
fn App() -> impl IntoView {
    provide_query_client();

    view! {
        <Router>
            <nav class=css::nav>
                <A href={"list"}>{"List of all recipes"}</A>
            </nav>
                <Routes>
                    <Route path="/" view=|| view! { <Home extra={"garfsmie".into()}/> }/>
                    <Route path="recipes" view=Mouse/>
                    <Route path="recipe/:name" view=RecipePageComponent/>
                    <Route path="list" view=RecipesComponent/>
            </Routes>
        </Router>
    }
}

#[component]
fn Mouse() -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    view! {
        <p>{x} " x " {y}</p>
    }
}

#[component]
fn Home(extra: String) -> impl IntoView {
    let s = include_str!("../public/recipes/egg_fried_rice.txt");
    let recipe = s.parse::<recipe_util::Recipe>().unwrap();

    console_log(&extra);

    view! {
        <RecipeComponent recipe/>
    }
}
