use patternfly_yew::prelude::{
    ActionGroup, Backdrop, Backdropper, Bullseye, Button, ButtonVariant, Content, DescriptionGroup,
    DescriptionList, Flex, FlexItem, Form, FormGroup, Modal, ModalVariant, SimpleSelect, Spinner,
    Stack, StackItem, TextInput, Tooltip,
};
use yew::{html, Component, Context, Html, Properties};
use yew_nested_router::prelude::RouterContext;

use crate::{
    api::{self, FetchError, FetchState},
    api_context::ApiContext,
    app::Route,
    menu_context::{MenuAction, MenuContext},
    slop::{recipe_svg, recipe_title},
};

pub enum Msg {
    SetRecipeFetchState(FetchState<api::Recipe>),
    SetTagsFetchState(FetchState<Vec<String>>),
    SetMenuSaveState(FetchState<()>),
    SetAddToBookState(FetchState<()>),
    FetchRecipe,
    FetchTags,
    AddModal(Vec<String>),
    NewTag(String),
    SelectTag(String),
    AddToBook,
    ApiUpdate(ApiContext),
    MenuUpdate(MenuContext),
    RouterUpdate(RouterContext<Route>),
    BackdropperUpdate(Backdropper),
    AddToMenu,
    RemoveFromMenu,
    SaveMenu,
}

enum State {
    Inactive,
    Pending,
    Success(api::Recipe),
    Failed(FetchError),
}

pub struct Recipe {
    state: State,
    saving_menu: bool,

    title: Option<String>,
    selected_tag: Option<String>,
    new_tag: Option<String>,

    id: String,
    api_context: ApiContext,
    menu_context: MenuContext,
    router: RouterContext<Route>,
    backdropper: Backdropper,
}

#[derive(Debug, Properties, PartialEq)]
pub struct RecipeProps {
    pub id: String,
}

impl Component for Recipe {
    type Message = Msg;
    type Properties = RecipeProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ApiUpdate))
            .expect("context should exist");
        let (menu_context, _) = ctx
            .link()
            .context::<MenuContext>(ctx.link().callback(Msg::MenuUpdate))
            .expect("context should exist");
        let (backdropper, _) = ctx
            .link()
            .context::<Backdropper>(ctx.link().callback(Msg::BackdropperUpdate))
            .expect("backdropper should exist");
        let (router, _) = ctx
            .link()
            .context::<RouterContext<Route>>(ctx.link().callback(Msg::RouterUpdate))
            .expect("context should exist");
        Self {
            state: State::Inactive,
            saving_menu: false,
            id: ctx.props().id.clone(),
            api_context,
            menu_context,
            router,
            title: None,
            backdropper,
            new_tag: None,
            selected_tag: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let api = self.api_context.api();
        let id = self.id.clone();
        match msg {
            Msg::SetRecipeFetchState(fetch_state) => {
                match fetch_state {
                    FetchState::NotFetching => self.state = State::Inactive,
                    FetchState::Fetching => self.state = State::Pending,
                    FetchState::Success(recipe) => {
                        if let Some(title) = recipe_title(recipe.source.as_str()) {
                            self.title = Some(title);
                        }
                        self.state = State::Success(recipe)
                    }
                    FetchState::Failed(e) => self.state = State::Failed(e),
                }
                true
            }
            Msg::SetTagsFetchState(fetch_state) => {
                match fetch_state {
                    FetchState::NotFetching => {}
                    FetchState::Fetching => {}
                    FetchState::Success(labels) => ctx.link().send_message(Msg::AddModal(labels)),
                    FetchState::Failed(e) => self.state = State::Failed(e),
                }
                true
            }
            Msg::SetMenuSaveState(fetch_state) => {
                match fetch_state {
                    FetchState::NotFetching => {}
                    FetchState::Fetching => {}
                    FetchState::Success(_) => self.saving_menu = false,
                    FetchState::Failed(e) => self.state = State::Failed(e),
                }
                true
            }
            Msg::SetAddToBookState(fetch_state) => match fetch_state {
                FetchState::NotFetching => false,
                FetchState::Fetching => false,
                FetchState::Success(_) => {
                    self.backdropper.close();
                    self.router.push(Route::Book);
                    true
                }
                FetchState::Failed(e) => {
                    self.state = State::Failed(e);
                    true
                }
            },
            Msg::FetchRecipe => {
                ctx.link().send_future(async move {
                    match api.fetch_recipe(id).await {
                        Ok(recipe) => Msg::SetRecipeFetchState(FetchState::Success(recipe)),
                        Err(err) => Msg::SetRecipeFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetRecipeFetchState(FetchState::Fetching));
                false
            }
            Msg::FetchTags => {
                ctx.link().send_future(async move {
                    match api.fetch_book_tags().await {
                        Ok(b) => Msg::SetTagsFetchState(FetchState::Success(b)),
                        Err(err) => Msg::SetTagsFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetTagsFetchState(FetchState::Fetching));
                false
            }
            Msg::NewTag(tag) => {
                self.new_tag = Some(tag);
                false
            }
            Msg::SelectTag(tag) => {
                self.selected_tag = Some(tag);
                true
            }
            Msg::AddToBook => {
                let recipe_id = self.id.clone();
                // TODO error on missing title or tag
                let title = self.title.clone().unwrap_or_default();
                let tag = self
                    .new_tag
                    .clone()
                    .unwrap_or_else(|| self.selected_tag.clone().unwrap_or_default());
                ctx.link().send_future(async move {
                    match api.create_book_entry(recipe_id, title, tag).await {
                        Ok(b) => Msg::SetAddToBookState(FetchState::Success(b)),
                        Err(err) => Msg::SetAddToBookState(FetchState::Failed(err)),
                    }
                });
                false
            }
            Msg::ApiUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
            Msg::RouterUpdate(router) => {
                self.router = router;
                false
            }
            Msg::MenuUpdate(menu_context) => {
                self.menu_context = menu_context;
                false
            }
            Msg::BackdropperUpdate(backdropper) => {
                self.backdropper = backdropper;
                false
            }
            Msg::AddModal(all_tags) => {
                let onclick = ctx.link().callback(|_| Msg::AddToBook);
                let onselect = ctx.link().callback(Msg::SelectTag);
                let onchange = ctx.link().callback(Msg::NewTag);
                let selected_tag = self.selected_tag.clone();
                self.backdropper.open(Backdrop {
                    content: html! {
                        <Bullseye>
                            <div class="recipe_modal">
                            <Modal
                                title = {"Add this recpie to your book "}
                                variant = { ModalVariant::Medium }
                            >
                                <Form>
                                    <FormGroup label="Select an existing tag" >
                                        <SimpleSelect<String> selected={selected_tag} entries={all_tags} {onselect}/>
                                    </FormGroup>
                                    <FormGroup label="Or create a new tag" >
                                        <TextInput {onchange} placeholder="breakfast"/>
                                    </FormGroup>
                                    <ActionGroup>
                                        <Button label="Add" variant={ButtonVariant::Primary} {onclick}/>
                                    </ActionGroup>
                                </Form>
                            </Modal>
                            </div>
                        </Bullseye>
                    },
                });
                true
            }
            Msg::AddToMenu => {
                if let State::Success(recipe) = &self.state {
                    self.menu_context.dispatch(MenuAction::Add(recipe.clone()));
                    self.saving_menu = true;
                }
                ctx.link().send_message(Msg::SaveMenu);
                true
            }
            Msg::RemoveFromMenu => {
                if let State::Success(recipe) = &self.state {
                    self.menu_context
                        .dispatch(MenuAction::Remove(recipe.id.clone()));
                    self.saving_menu = true;
                }
                ctx.link().send_message(Msg::SaveMenu);
                true
            }
            Msg::SaveMenu => {
                let menu = self.menu_context.menu();
                ctx.link().send_future(async move {
                    match api.create_menu(menu).await {
                        Ok(r) => Msg::SetMenuSaveState(FetchState::Success(r)),
                        Err(err) => Msg::SetMenuSaveState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetMenuSaveState(FetchState::Fetching));
                false
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
            State::Success(recipe) => {
                let onclick = ctx.link().callback(|_| Msg::FetchTags);
                let title = self.title.clone().unwrap_or_default();
                let card = Html::from_html_unchecked(
                    recipe_svg(&recipe.source).unwrap_or_default().into(),
                );

                let count = self.menu_context.count_recipe(&recipe.id);

                let add_to_menu = {
                    let onclick = ctx.link().callback(|_| Msg::AddToMenu);
                    let label = if count > 0 {
                        format!("Add to Menu ({count})")
                    } else {
                        "Add to Menu".to_string()
                    };
                    html! {
                        <Tooltip text={"Add this recipe to the menu. A recipe can be added multiple times."}>
                            <Button
                                {label}
                                variant={ButtonVariant::Secondary}
                                loading={self.saving_menu}
                                {onclick}
                            />
                        </Tooltip>
                    }
                };
                let remove_from_menu = {
                    let onclick = ctx.link().callback(|_| Msg::RemoveFromMenu);
                    let disabled = count == 0;
                    let text = if disabled {
                        "This recipe is not part of the menu."
                    } else {
                        "Remove all occurences of this recipe from the menu."
                    };
                    html! {
                        <Tooltip {text}>
                            <Button
                                label="Remove from Menu"
                                variant={ButtonVariant::Secondary}
                                loading={self.saving_menu}
                                {disabled}
                                {onclick}
                            />
                        </Tooltip>
                    }
                };

                html! {
                    <Stack>
                        <StackItem>
                            <Content >
                                <h2>{title}</h2>
                            </Content>
                            <DescriptionList>
                                <DescriptionGroup term="Author">
                                    {recipe.author.id.clone()}
                                </DescriptionGroup>
                            </DescriptionList>
                            <div>{card}</div>
                        </StackItem>
                        <StackItem>
                            <Flex>
                                <FlexItem>
                                    <Button
                                        label="Add to Recipe Book"
                                        variant={ButtonVariant::Primary}
                                        {onclick}
                                    />
                                </FlexItem>
                                <FlexItem>
                                    {add_to_menu}
                                </FlexItem>
                                <FlexItem>
                                    {remove_from_menu}
                                </FlexItem>
                            </Flex>
                        </StackItem>
                    </Stack>
                }
            }
            State::Failed(err) => html! { err },
        }
    }
}
