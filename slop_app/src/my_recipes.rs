use patternfly_yew::prelude::*;
use yew::{function_component, html, Component, Context, Html, Properties};
use yew_nested_router::components::Link;

use crate::{api, app::Route};
use crate::{
    api::FetchState, api_context::ApiContext, recipe_link::RecipeLink, slop::recipe_title,
};

pub enum Msg {
    SetFetchState(FetchState<Vec<api::Recipe>>),
    Fetch,
    ContextUpdate(ApiContext),
}

pub struct MyRecipes {
    fetch_state: FetchState<Vec<api::Recipe>>,
    api_context: ApiContext,
}

impl Component for MyRecipes {
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
            Msg::Fetch => {
                ctx.link().send_future(async move {
                    match api.fetch_my_recipes().await {
                        Ok(md) => Msg::SetFetchState(FetchState::Success(md)),
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
                ctx.link().send_message(Msg::Fetch);
                html! {
                        <Spinner/>
                }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(recipes) => {
                let children = recipes
                    .iter()
                    .map(|r| html! { <RecipeItem recipe={r.clone()}/> });
                html! {
                    <Stack gutter=true>
                        <StackItem>
                            <Link<Route> target={Route::NewRecipe}>
                                <Button variant={ButtonVariant::Primary} icon={Icon::PlusCircle} label="New Recipe" ></Button>
                            </Link<Route>>
                        </StackItem>
                        <StackItem><Stack gutter=true> {children.collect::<Html>() } </Stack></StackItem>
                    </Stack>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

#[derive(Properties, PartialEq)]
struct RecipeItemProps {
    recipe: api::Recipe,
}
#[function_component(RecipeItem)]
fn recipe_item(props: &RecipeItemProps) -> Html {
    let title = recipe_title(&props.recipe.source);

    html! {
        <>
        <StackItem>
            <RecipeLink id={props.recipe.id.clone()} >
                <Content><h2>{title}</h2></Content>
            </RecipeLink>
        </StackItem>
        <Divider/>
        </>
    }
}
