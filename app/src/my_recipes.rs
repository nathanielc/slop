use patternfly_yew::prelude::*;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_nested_router::components::Link;

use crate::{
    api,
    app::Route,
    pagination::{self, Paginator},
};
use crate::{
    api::FetchState, api_context::ApiContext, recipe_link::RecipeLink, slop::recipe_title,
};

const RECIPES_PER_PAGE: usize = 10;

pub enum Msg {
    SetFetchState(FetchState<api::Recipes>),
    Fetch(api::Page),
    ApiUpdate(ApiContext),
}

pub struct MyRecipes {
    fetch_state: FetchState<api::Recipes>,
    api_context: ApiContext,
    current_page: api::Page,
}

impl Component for MyRecipes {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ApiUpdate))
            .expect("context should exist");
        Self {
            fetch_state: FetchState::NotFetching,
            api_context,
            current_page: api::Page {
                first: Some(RECIPES_PER_PAGE),
                ..Default::default()
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            Msg::Fetch(page) => {
                let api = self.api_context.api();
                ctx.link().send_future(async move {
                    match api.fetch_my_recipes(&page).await {
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
            Msg::ApiUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::NotFetching => {
                ctx.link()
                    .send_message(Msg::Fetch(self.current_page.clone()));
                html! {
                        <Spinner/>
                }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(recipes) => {
                let page = self.current_page.clone();
                let ondelete = ctx.link().callback(move |_| Msg::Fetch(page.clone()));
                let children = recipes.recipes.iter().map(
                    |r| html! { <RecipeItem recipe={r.clone()} ondelete={ondelete.clone()} /> },
                );
                let onpage = ctx.link().callback(Msg::Fetch);
                let page_buttons = html! {
                    <Paginator limit={RECIPES_PER_PAGE} page_info={recipes.page_info.clone()} {onpage} />
                };
                html! {
                    <div class="my-recipes">
                        <Stack gutter=true>
                            <StackItem>
                                <Link<Route> target={Route::NewRecipe}>
                                    <Button variant={ButtonVariant::Primary} icon={Icon::PlusCircle} label="New Recipe" ></Button>
                                </Link<Route>>
                            </StackItem>
                            <StackItem>
                                {page_buttons.clone()}
                            </StackItem>
                            <StackItem>
                                <table>
                                    <thead>
                                        <th/>
                                        <th/>
                                    </thead>
                                    <tbody>
                                        {for children}
                                    </tbody>
                                </table>
                            </StackItem>
                            <StackItem>
                                {page_buttons}
                            </StackItem>
                        </Stack>
                    </div>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

#[derive(Properties, PartialEq)]
struct RecipeItemProps {
    recipe: api::Recipe,
    ondelete: Callback<()>,
}

enum RecipeItemMsg {
    SetDeleteState(FetchState<()>),
    Delete,
    ApiUpdate(ApiContext),
}

struct RecipeItem {
    fetch_state: FetchState<()>,
    api_context: ApiContext,
    title: String,
}

impl Component for RecipeItem {
    type Message = RecipeItemMsg;

    type Properties = RecipeItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        let title = recipe_title(&ctx.props().recipe.source).unwrap_or_default();
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(RecipeItemMsg::ApiUpdate))
            .expect("context should exist");
        Self {
            title,
            fetch_state: FetchState::NotFetching,
            api_context,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RecipeItemMsg::SetDeleteState(fetch_state) => {
                self.fetch_state = fetch_state;
                if matches!(&self.fetch_state, FetchState::Success(_)) {
                    ctx.props().ondelete.emit(());
                }
                true
            }
            RecipeItemMsg::Delete => {
                let api = self.api_context.api();
                let id = ctx.props().recipe.id.clone();
                ctx.link().send_future(async move {
                    match api
                        .update_recipe(
                            &id,
                            &api::RecipeUpdate {
                                deleted: Some(true),
                                ..Default::default()
                            },
                        )
                        .await
                    {
                        Ok(_) => RecipeItemMsg::SetDeleteState(FetchState::Success(())),
                        Err(err) => RecipeItemMsg::SetDeleteState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(RecipeItemMsg::SetDeleteState(FetchState::Fetching));
                false
            }
            RecipeItemMsg::ApiUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let loading = matches!(self.fetch_state, FetchState::Fetching);
        let onclick = ctx.link().callback(|_| RecipeItemMsg::Delete);
        html! {
            <tr>
                <td>
                    <RecipeLink id={ctx.props().recipe.id.clone()} >{self.title.clone()}</RecipeLink>
                </td>
                <td>
                    <Bullseye>
                        <Button {loading} variant={ButtonVariant::Plain} icon={Icon::Trash} {onclick}/>
                    </Bullseye>
                </td>
            </tr>
        }
    }
}
