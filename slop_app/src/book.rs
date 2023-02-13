use log::debug;
use patternfly_yew::prelude::*;
use yew::{
    function_component, html, html_nested, virtual_dom::VChild, Component, Context, Html,
    Properties,
};

use crate::{
    api::{self, FetchState},
    api_context::ApiContext,
    recipe_link::RecipeLink,
};

#[derive(Debug)]
pub enum BookMsg {
    SetFetchState(FetchState<Vec<String>>),
    Fetch,
    ContextUpdate(ApiContext),
    SelectTab(usize),
}
pub struct Book {
    selected: usize,
    fetch_state: FetchState<Vec<String>>,
    api_context: ApiContext,
}
impl Component for Book {
    type Message = BookMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(BookMsg::ContextUpdate))
            .expect("context should exist");
        Self {
            selected: 0,
            fetch_state: FetchState::NotFetching,
            api_context,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let api = self.api_context.api();
        match msg {
            BookMsg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            BookMsg::Fetch => {
                ctx.link().send_future(async move {
                    match api.fetch_book_tags().await {
                        Ok(tags) => BookMsg::SetFetchState(FetchState::Success(tags)),
                        Err(err) => BookMsg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(BookMsg::SetFetchState(FetchState::Fetching));
                false
            }
            BookMsg::ContextUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
            BookMsg::SelectTab(selected) => {
                self.selected = selected;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::NotFetching => {
                ctx.link().send_message(BookMsg::Fetch);
                html! {
                        <Spinner/>
                }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(data) => {
                let onselect = ctx.link().callback(|index| BookMsg::SelectTab(index));
                let mut data = data.to_owned();
                data.sort();
                let tabs = data
                    .iter()
                    .enumerate()
                    .map(|(i, tag)| {
                        let capital_tag = capitalize(tag);
                        html_nested! {
                            <Tab<usize> index={i} title={capital_tag}>
                                <BookTab tag={tag.clone()} />
                            </Tab<usize>>
                        }
                    })
                    .collect::<Vec<VChild<Tab<usize>>>>();
                html! {
                    <Tabs<usize> selected={self.selected} {onselect}>
                    { tabs }
                    </Tabs<usize>>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    if let Some(f) = c.next() {
        f.to_uppercase().collect::<String>() + c.as_str()
    } else {
        String::new()
    }
}

#[derive(Debug)]
pub enum BookTabMsg {
    SetFetchState(FetchState<api::BookEntries>),
    Fetch,
    ContextUpdate(ApiContext),
}
pub struct BookTab {
    fetch_state: FetchState<api::BookEntries>,
    api_context: ApiContext,
}

#[derive(Debug, Properties, PartialEq)]
pub struct BookTabProps {
    tag: String,
}

impl Component for BookTab {
    type Message = BookTabMsg;
    type Properties = BookTabProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(BookTabMsg::ContextUpdate))
            .expect("context should exist");
        Self {
            fetch_state: FetchState::NotFetching,
            api_context,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        debug!("BookTab update {:?}", msg);
        let api = self.api_context.api();
        match msg {
            BookTabMsg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            BookTabMsg::Fetch => {
                let tag = ctx.props().tag.clone();
                ctx.link().send_future(async move {
                    match api.fetch_book_entries(tag).await {
                        Ok(entries) => BookTabMsg::SetFetchState(FetchState::Success(entries)),
                        Err(err) => BookTabMsg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(BookTabMsg::SetFetchState(FetchState::Fetching));
                false
            }
            BookTabMsg::ContextUpdate(api_context) => {
                self.api_context = api_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::NotFetching => {
                ctx.link().send_message(BookTabMsg::Fetch);
                html! {
                        <Spinner/>
                }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(data) => {
                let entries = data
                    .entries
                    .iter()
                    .map(|book_entry| {
                        html! {
                            <BookEntry book_entry={book_entry.clone()} />
                        }
                    })
                    .collect::<Vec<Html>>();
                html! {
                    <div class="book-tab">
                        <List>
                        { entries }
                        </List>
                    </div>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

#[derive(Debug, Properties, PartialEq)]
struct BookEntryProps {
    book_entry: api::BookEntry,
}

#[function_component(BookEntry)]
fn book_entry(props: &BookEntryProps) -> Html {
    html! {
        <RecipeLink id={props.book_entry.recipe_id.clone()}>{props.book_entry.title.clone()}</RecipeLink>
    }
}
