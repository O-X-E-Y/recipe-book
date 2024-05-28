use crate::{measurements::{Imperial, Metric}, recipe_util::*};

use gloo_net::http::Request;
use include_dir::include_dir;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_query::*;
use leptos_router::*;
use once_cell::sync::Lazy;
use stylance::{classes, import_crate_style};

import_crate_style!(pub css, "./styles/recipe.module.css");

pub static RECIPES: Lazy<Vec<String>> = Lazy::new(|| {
    include_dir!("./public/recipes")
        .entries()
        .into_iter()
        .flat_map(|e| e.path().file_prefix())
        .flat_map(|s| s.to_str())
        .map(Into::into)
        .collect()
});

#[component]
pub fn IntroductionComponent(image: Option<Image>, introduction: Option<String>) -> impl IntoView {
    view! {
        <ImageComponent image={image}/>
        <p class=classes!(css::introduction, css::content)>{introduction}</p>
    }
}

#[component]
pub fn ImageComponent(image: Option<Image>) -> impl IntoView {
    if let Some(image) = image {
        Some(view! {
            <img href={image.href}/>
        })
    } else {
        None
    }
}

#[component]
pub fn UnitButtonComponent() -> impl IntoView {
    let unit = use_context::<ReadSignal<bool>>()
        .expect("We know this signal to be provided");

    let unit_setter = use_context::<WriteSignal<bool>>()
        .expect("We know this signal to be provided");

    let unit_str = move || match unit() {
        true => "Metric",
        false => "Imperial"
    };

    view! {
        <div class=css::unit_button_wrapper>
            <label name="unit-button">
                <button
                    class=css::unit_button
                    on:click={ move |_| unit_setter.update(|b| *b = !*b) }
                >
                    { unit_str }
                </button>
            </label>
        </div>
    }
}

#[component]
pub fn IngredientsComponent(ingredients: Vec<Ingredient>) -> impl IntoView {
    let (unit, unit_setter) = create_signal(true);
    
    provide_context(unit);
    provide_context(unit_setter);

    let ingredients = move || ingredients
        .into_iter()
        .map(|i| {
            match unit() {
                true => i.as_metric().to_string(),
                false => i.as_imperial().to_string(),
            }
        })
        .collect_view();

    view! {
        <h2 class=css::subheader>{"Ingredients:"}</h2>
        <div class=classes!(css::ingredient_list, css::content)>
            <UnitButtonComponent/>
            <ul>
                {ingredients()}
            </ul>
        </div>
    }
}

#[component]
pub fn StepsComponent(steps: Vec<Step>) -> impl IntoView {
    view! {
        <h2 class=css::subheader>{"Steps:"}</h2>
        <ol class=classes!(css::step_list, css::content)>
            {steps
                .into_iter()
                .map(|s| view! { <li>{s.body}</li> })
                .collect_view()
            }
        </ol>
    }
}

#[component]
pub fn RecipeComponent(recipe: Recipe) -> impl IntoView {
    view! {
        <h1 class=css::header>{recipe.title}</h1>
        <IntroductionComponent image={recipe.image} introduction={recipe.introduction}/>
        <IngredientsComponent ingredients={recipe.ingredients}/>
        <StepsComponent steps={recipe.steps}/>
    }
}

#[derive(Debug, Clone, Default, Params, PartialEq)]
pub struct RecipeParams {
    name: String,
}

async fn load_recipe(url: String) -> Result<Recipe, RecipeError> {
    Request::get(&url)
        .send()
        .await
        .map_err(|e| RecipeError::CustomString(e.to_string()))?
        .text()
        .await
        .map_err(|e| RecipeError::CustomString(e.to_string()))?
        .parse()
}

#[component]
pub fn RecipePageComponent() -> impl IntoView {
    let params = use_params::<RecipeParams>();
    let query_scope = create_query(load_recipe, QueryOptions::default());

    let name = move || params.with_untracked(|params| params.clone().unwrap_or_default().name);
    let url = move || format!("../recipes/{}.txt", name());
    let QueryResult { data, .. } = query_scope.use_query(url);

    view! {
        <div class=css::recipe>
            <Transition
                fallback=move || {
                    view! { <h2>"Loading..."</h2> }
                }
            >
                {move || {
                    data
                        .get()
                        .and_then(|res| res.ok())
                        .map(|recipe| {
                            view! { <RecipeComponent recipe/> }
                        })
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn RecipesComponent() -> impl IntoView {
    console_log(&format!("{RECIPES:?} yay ay ay ay ay"));

    let url = |s: &str| format!("/recipe/{s}");

    view! {
        <ul>
            {RECIPES
                .iter()
                .map(|i| view! {
                    <li><A href={url(i)}>{i.to_string()}</A></li>
                })
                .collect_view()
            }
        </ul>
    }
}
