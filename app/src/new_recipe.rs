use patternfly_yew::prelude::{
    Button, ButtonVariant, Content, DescriptionGroup, DescriptionList, Icon, Spinner, Stack,
    StackItem,
};
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

use crate::{api::FetchState, api_context::ApiContext, slop::recipe_title};

pub enum Msg {
    SetFetchState(FetchState<()>),
    CreateRecipe,
    Update,
    ContextUpdate(ApiContext),
}

pub struct NewRecipe {
    fetch_state: FetchState<()>,
    node_ref: NodeRef,
    api_context: ApiContext,

    source: String,
    title: String,
}

impl Component for NewRecipe {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ContextUpdate))
            .expect("context should exist");
        Self {
            fetch_state: FetchState::NotFetching,
            node_ref: NodeRef::default(),
            source: "".to_string(),
            title: "".to_string(),
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
            Msg::CreateRecipe => {
                let textarea = self.node_ref.cast::<HtmlInputElement>();
                if let Some(textarea) = textarea {
                    let source = textarea.value();
                    ctx.link().send_future(async move {
                        match api.create_recipe(source).await {
                            Ok(s) => Msg::SetFetchState(FetchState::Success(s)),
                            Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                        }
                    });
                    ctx.link()
                        .send_message(Msg::SetFetchState(FetchState::Fetching));
                }
                false
            }
            Msg::Update => {
                let textarea = self.node_ref.cast::<HtmlInputElement>();
                if let Some(textarea) = textarea {
                    let src = textarea.value();
                    if let Some(title) = recipe_title(src.as_str()) {
                        self.title = title;
                    }
                    self.source = src;
                    true
                } else {
                    false
                }
            }
            Msg::ContextUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::NotFetching => html! {
                <Stack gutter=true>
                    <StackItem>
                        <DescriptionList>
                            <DescriptionGroup term="Title">
                                <label for="source-input">
                                    {self.title.clone()}
                                </label>
                            </DescriptionGroup>
                        </DescriptionList>
                    </StackItem>
                    <StackItem>
                        <textarea
                            ref={self.node_ref.clone()}
                            id="source-input"
                            type="text"
                            oninput={ctx.link().callback(|_| Msg::Update)}
                            value={self.source.clone()}
                            rows=20
                            cols=80
                        />
                    </StackItem>
                    <StackItem>
                        <Button variant={ButtonVariant::Primary} icon={Icon::PlusCircle} label="Create New Recipe" onclick={ctx.link().callback(|_| Msg::CreateRecipe)}/>
                    </StackItem>
                </Stack>
            },
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(_) => html! { <Content>{"Created recipe"}</Content> },
            FetchState::Failed(err) => html! { err },
        }
    }
}
