use patternfly_yew::prelude::*;
use yew::{function_component, html, Component, Context, Html, Properties};
use yew_nested_router::components::Link;

use crate::{
    api::{self, FetchState},
    api_context::ApiContext,
    app::Route,
    pagination::{self, Paginator},
    recipe_link::RecipeLink,
    slop::recipe_title,
};

const RECIPES_PER_PAGE: usize = 10;

pub enum Msg {
    SetFetchState(FetchState<api::Recipes>),
    Fetch(api::Page),
    ContextUpdate(ApiContext),
}

pub struct Discover {
    fetch_state: FetchState<api::Recipes>,
    api_context: ApiContext,
}

impl Component for Discover {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ContextUpdate))
            .expect("context should exist");
        Self {
            fetch_state: FetchState::NotFetching,
            api_context,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let api = self.api_context.api();
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            Msg::Fetch(page) => {
                ctx.link().send_future(async move {
                    match api.fetch_all_recipes(&page).await {
                        Ok(mut ret) => {
                            pagination::apply_remembered_direction(&page, &mut ret.page_info);
                            Msg::SetFetchState(FetchState::Success(ret))
                        }
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::ContextUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::NotFetching => {
                ctx.link().send_message(Msg::Fetch(api::Page {
                    first: Some(RECIPES_PER_PAGE),
                    ..Default::default()
                }));
                html! { <Spinner/> }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(recipes) => {
                if recipes.recipes.is_empty() {
                    let my_recipes =
                        html! {<Link<Route> target={Route::MyRecipes}>{"here"}</Link<Route>>};
                    html! {
                        <Content>
                            <p>
                                {"There are no recipes. Be the first to create a recipe "}
                                {my_recipes}
                                {"."}
                            </p>
                        </Content>
                    }
                } else {
                    let children = recipes
                        .recipes
                        .iter()
                        .map(|r| html! { <RecipeItem recipe={r.clone()}/> });

                    let onpage = ctx.link().callback(Msg::Fetch);
                    let page_buttons = html! {
                        <Paginator limit={RECIPES_PER_PAGE} page_info={recipes.page_info.clone()} {onpage} />
                    };

                    html! {
                        <Stack gutter=true>
                            <StackItem>
                                {page_buttons.clone()}
                            </StackItem>
                            {for children }
                            <StackItem>
                                {page_buttons}
                            </StackItem>
                        </Stack>
                    }
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RecipeItemProps {
    recipe: api::Recipe,
}
#[function_component(RecipeItem)]
pub fn recipe_item(props: &RecipeItemProps) -> Html {
    let title = recipe_title(&props.recipe.source);

    html! {
        <>
        <StackItem>
            <RecipeLink id={props.recipe.id.clone()}>
                <Content>
                <h2>{title}</h2>
                </Content>
                <DescriptionList>
                    <DescriptionGroup term="Author">
                        <Content>{props.recipe.author.id.clone()}</Content>
                    </DescriptionGroup>
                </DescriptionList>
            </RecipeLink>
        </StackItem>
        <Divider/>
        </>
    }
}
