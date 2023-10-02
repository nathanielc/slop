use yew::{function_component, html, Children, Html, Properties};
use yew_nested_router::components::Link;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct RecipeLinkProps {
    pub children: Children,
    pub id: String,
}

#[function_component(RecipeLink)]
pub fn recipe_link(props: &RecipeLinkProps) -> Html {
    html! {
        <Link<Route> target={Route::Recipe{id: props.id.clone()}}>
            { for props.children.iter() }
        </Link<Route>>
    }
}
