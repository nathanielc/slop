use std::rc::Rc;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use log::warn;
use yew::prelude::*;

use crate::api;

const API_ADDRESS_KEY: &str = "slop-api-address";
const DEFAULT_API_ADDRESS: &str = "http://localhost:7007";

#[derive(Debug, PartialEq, Clone)]
pub struct ApiHandle {
    handle: Rc<api::ApiHandle>,
    address: String,
}

impl ApiHandle {
    fn new(address: String) -> Self {
        Self {
            address: address.clone(),
            handle: Rc::new(api::ApiHandle::new(address)),
        }
    }
    pub fn api(&self) -> Rc<api::ApiHandle> {
        Rc::clone(&self.handle)
    }
    pub fn address(&self) -> &str {
        &self.address
    }
}

pub type ApiContext = UseReducerHandle<ApiHandle>;

impl Reducible for ApiHandle {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match LocalStorage::set(API_ADDRESS_KEY, &action) {
            Ok(_) => {}
            Err(err) => warn!("error saving {API_ADDRESS_KEY}: {err}"),
        };
        Self::new(action).into()
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ApiProviderProps {
    pub children: Children,
}

#[function_component]
pub fn ApiProvider(props: &ApiProviderProps) -> Html {
    let api = use_reducer(|| {
        let address = match LocalStorage::get(API_ADDRESS_KEY) {
            Ok(address) => address,
            Err(StorageError::KeyNotFound(_)) => DEFAULT_API_ADDRESS.to_string(),
            Err(err) => {
                warn!("error loading {API_ADDRESS_KEY}: {err}");
                DEFAULT_API_ADDRESS.to_string()
            }
        };
        ApiHandle::new(address)
    });

    html! {
        <ContextProvider<ApiContext> context={api}>
            {props.children.clone()}
        </ContextProvider<ApiContext>>
    }
}
