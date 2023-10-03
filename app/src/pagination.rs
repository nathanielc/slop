use log::debug;
use patternfly_yew::prelude::{Button, ButtonVariant, Flex, FlexItem, Icon};
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::api;

#[derive(Properties, PartialEq)]
pub struct PaginatorProps {
    pub limit: usize,
    pub page_info: Option<api::PageInfo>,
    pub onpage: Callback<api::Page>,
}
pub struct Paginator;

pub enum Msg {
    Page(api::Page),
}

// We directly implement the Component trait because of the dynamic nature in which callbacks are
// created.
// Otherwise we would leverage `use_callback`.
impl Component for Paginator {
    type Message = Msg;

    type Properties = PaginatorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Page(page) => ctx.props().onpage.emit(page),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let limit = ctx.props().limit;
        let (prev_page, next_page) = if let Some(page_info) = &ctx.props().page_info {
            debug!("page info {page_info:?}");
            let prev_page = if page_info.has_previous_page {
                let start = page_info.start_cursor.to_owned();
                Some(ctx.link().callback(move |_| {
                    Msg::Page(api::Page {
                        direction: Some(api::Direction::Backward),
                        before: Some(start.clone()),
                        last: Some(limit),
                        ..Default::default()
                    })
                }))
            } else {
                None
            };

            let next_page = if page_info.has_next_page {
                let end = page_info.end_cursor.to_owned();
                Some(ctx.link().callback(move |_| {
                    Msg::Page(api::Page {
                        direction: Some(api::Direction::Forward),
                        after: Some(end.clone()),
                        first: Some(limit),
                        ..Default::default()
                    })
                }))
            } else {
                None
            };
            (prev_page, next_page)
        } else {
            (None, None)
        };
        let prev_disabled = prev_page.is_none();
        let next_disabled = next_page.is_none();

        html! {
            <Flex>
                <FlexItem>
                    <Button disabled={prev_disabled} onclick={prev_page.clone().unwrap_or_else(Callback::noop)} variant={ButtonVariant::Plain} icon={Icon::AngleLeft} />
                </FlexItem>
                <FlexItem>
                    <Button disabled={next_disabled} onclick={next_page.clone().unwrap_or_else(Callback::noop)} variant={ButtonVariant::Plain} icon={Icon::AngleRight} />
                </FlexItem>
            </Flex>
        }
    }
}

/// Updates the PageInfo's has_next_page and has_previous_page fields with the remebered direction
/// the page moved.
///
/// The database cannot efficiently know if there are more records in the oppososite direction is it
/// reading. Therefore here we assume if we moved in a direction, that we came from something and
/// therefore it's safe to update the has_next_page and has_previous_page fields accordingly.
pub fn apply_remembered_direction(page: &api::Page, page_info: &mut Option<api::PageInfo>) {
    if let Some(dir) = &page.direction {
        match dir {
            api::Direction::Forward => page_info.as_mut().map(|pi| pi.has_previous_page = true),
            api::Direction::Backward => page_info.as_mut().map(|pi| pi.has_next_page = true),
        };
    };
    // else, we didn't move in a direction i.e. this is the first page request
    // we cannot assume anything.
}
