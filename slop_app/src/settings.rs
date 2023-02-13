use log::debug;
use patternfly_yew::prelude::*;
use yew::{html, Component, Context, Html};

use crate::api_context::ApiContext;

pub enum Msg {
    SetAddress(String),
    Save,
    ContextUpdate(ApiContext),
}

pub struct Settings {
    address: String,
    api_context: ApiContext,
}

impl Component for Settings {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ContextUpdate))
            .expect("context should exist");
        Self {
            address: api_context.address().to_owned(),
            api_context,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextUpdate(api_context) => {
                debug!("ContextUpdate {}", api_context.address());
                self.api_context = api_context;
                self.address = self.api_context.address().to_owned();
                false
            }
            Msg::SetAddress(addr) => {
                self.address = addr;
                false
            }
            Msg::Save => {
                debug!("saving new address");
                self.api_context.dispatch(self.address.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value = self.address.clone();
        let onchange = ctx.link().callback(|addr| Msg::SetAddress(addr));
        let onclick = ctx.link().callback(|_| Msg::Save);
        html! {
            <Form>
                <FormGroup
                    label="Server address"
                    required=true
                >
                    <TextInput {onchange} required=true {value}/>
                </FormGroup>
                <ActionGroup>
                    <Button {onclick} variant={ButtonVariant::Primary} label="Save"/>
                </ActionGroup>
            </Form>
        }
    }
}
