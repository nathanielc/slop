use patternfly_yew::prelude::{
    Button, ButtonVariant, Content, Flex, FlexItem, Spinner, Stack, StackItem,
};
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_nested_router::prelude::RouterContext;

use crate::{
    api::{self, FetchError, FetchState},
    api_context::ApiContext,
    app::Route,
    slop::recipe_title,
};

pub enum Msg {
    SetFetchState(FetchState<api::Recipe>),
    SetSaveState(FetchState<()>),
    FetchRecipe,
    UpdateSource,
    Save,
    ApiUpdate(ApiContext),
    RouterUpdate(RouterContext<Route>),
}

enum State {
    Inactive,
    Pending,
    Success,
    Failed(FetchError),
}

pub struct EditRecipe {
    state: State,
    id: String,
    node_ref: NodeRef,
    api_context: ApiContext,
    router: RouterContext<Route>,

    source: Option<String>,
    title: Option<String>,
    saving: bool,
}

#[derive(Debug, Properties, PartialEq)]
pub struct EditRecipeProps {
    pub id: String,
}

impl Component for EditRecipe {
    type Message = Msg;
    type Properties = EditRecipeProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ApiUpdate))
            .expect("context should exist");
        let (router, _) = ctx
            .link()
            .context::<RouterContext<Route>>(ctx.link().callback(Msg::RouterUpdate))
            .expect("context should exist");
        Self {
            state: State::Inactive,
            id: ctx.props().id.clone(),
            node_ref: Default::default(),
            api_context,
            router,
            title: None,
            source: None,
            saving: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                match fetch_state {
                    FetchState::NotFetching => self.state = State::Inactive,
                    FetchState::Fetching => self.state = State::Pending,
                    FetchState::Success(recipe) => {
                        self.title = recipe_title(&recipe.source);
                        self.source = Some(recipe.source);
                        self.state = State::Success;
                    }
                    FetchState::Failed(e) => self.state = State::Failed(e),
                }
                true
            }
            Msg::SetSaveState(fetch_state) => {
                match fetch_state {
                    FetchState::NotFetching | FetchState::Fetching => {}
                    FetchState::Success(()) => {
                        self.router.push(Route::Recipe {
                            id: self.id.clone(),
                        });
                    }
                    FetchState::Failed(e) => self.state = State::Failed(e),
                }
                true
            }
            Msg::FetchRecipe => {
                let api = self.api_context.api();
                let id = self.id.clone();
                ctx.link().send_future(async move {
                    match api.fetch_recipe(id).await {
                        Ok(recipe) => Msg::SetFetchState(FetchState::Success(recipe)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::UpdateSource => {
                let textarea = self.node_ref.cast::<HtmlInputElement>();
                if let Some(textarea) = textarea {
                    let src = textarea.value();
                    self.title = recipe_title(src.as_str());
                    self.source = Some(src);
                    true
                } else {
                    false
                }
            }
            Msg::ApiUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
            Msg::RouterUpdate(router) => {
                self.router = router;
                false
            }
            Msg::Save => {
                self.saving = true;
                let textarea = self.node_ref.cast::<HtmlInputElement>();
                if let Some(textarea) = textarea {
                    let api = self.api_context.api();
                    let source = textarea.value();
                    let id = self.id.clone();
                    let update = api::RecipeUpdate {
                        source: Some(source),
                        ..Default::default()
                    };
                    ctx.link().send_future(async move {
                        match api.update_recipe(&id, &update).await {
                            Ok(s) => Msg::SetSaveState(FetchState::Success(s)),
                            Err(err) => Msg::SetSaveState(FetchState::Failed(err)),
                        }
                    });
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Inactive => {
                ctx.link().send_message(Msg::FetchRecipe);
                html! { <Spinner /> }
            }
            State::Pending => html! { <Spinner/> },
            State::Success => {
                let title = self.title.clone().unwrap_or_default();
                let source = self.source.clone().unwrap_or_default();
                let onclick = ctx.link().callback(|_| Msg::Save);

                html! {
                    <Stack>
                        <StackItem>
                            <div class="recipe-description">
                                <Content >
                                    <h2>{title}</h2>
                                </Content>
                            </div>
                        </StackItem>
                        <StackItem>
                            <textarea
                                ref={self.node_ref.clone()}
                                id="source-input"
                                type="text"
                                oninput={ctx.link().callback(|_| Msg::UpdateSource)}
                                value={source}
                                rows=20
                                cols=80
                            />
                        </StackItem>
                        <StackItem>
                            <div class="recipe-controls">
                                <Flex>
                                    <FlexItem>
                                        <Button
                                            label="Save"
                                            variant={ButtonVariant::Primary}
                                            loading={self.saving}
                                            {onclick}
                                        />
                                    </FlexItem>
                                </Flex>
                            </div>
                        </StackItem>
                    </Stack>
                }
            }
            State::Failed(err) => html! { err },
        }
    }
}
