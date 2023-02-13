use std::{cell::RefCell, rc::Rc};

use slop::{menu::aggregate_ingredients, parse, semantic::convert_source_file};
use yew::prelude::*;

use crate::{
    api::{self, FetchState, MenuRecipe},
    api_context::ApiContext,
    slop::recipe_title,
};

#[derive(Debug, PartialEq, Clone)]
pub struct MenuHandle {
    menu: RefCell<api::MenuCreate>,
}

impl MenuHandle {
    fn new() -> Self {
        Self {
            menu: api::MenuCreate {
                recipes: Vec::new(),
                ingredients: Vec::new(),
            }
            .into(),
        }
    }
    pub fn menu(&self) -> api::MenuCreate {
        self.menu.borrow().clone()
    }
    pub fn count_recipe(&self, recipe_id: &str) -> usize {
        self.menu
            .borrow()
            .recipes
            .iter()
            .filter(|recipe| recipe.id == recipe_id)
            .count()
    }
}

pub type MenuContext = UseReducerHandle<MenuHandle>;

pub enum MenuAction {
    Set(api::MenuCreate),
    Add(api::Recipe),
    Remove(String),
}

impl Reducible for MenuHandle {
    type Action = MenuAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MenuAction::Set(m) => {
                let mut menu = self.menu.borrow_mut();
                *menu = m;
            }
            MenuAction::Add(recipe) => self.menu.borrow_mut().recipes.push(api::MenuRecipe {
                id: recipe.id,
                title: recipe_title(&recipe.source).unwrap_or_default(),
                source: recipe.source,
            }),
            MenuAction::Remove(recipe_id) => {
                let mut menu = self.menu.borrow_mut();
                let recipes: Vec<&MenuRecipe> = menu
                    .recipes
                    .iter()
                    .filter(|recipe| recipe.id != recipe_id)
                    .collect();

                if recipes.len() != menu.recipes.len() {
                    menu.recipes = recipes.into_iter().cloned().collect();
                }
            }
        }
        // Always update the set of ingredients
        {
            let mut menu = self.menu.borrow_mut();
            let recipes = menu
                .recipes
                .iter()
                .flat_map(|r| convert_source_file(parse(&r.source).unwrap()).recipes);
            menu.ingredients = aggregate_ingredients(recipes)
                .into_iter()
                .map(|(name, amounts)| api::MenuIngredient {
                    name,
                    amount: amounts.to_string(),
                })
                .collect::<Vec<api::MenuIngredient>>();
        }
        self
    }
}

#[derive(Debug)]
pub enum Msg {
    SetFetchState(FetchState<api::Menu>),
    Fetch,
    ApiUpdate(ApiContext),
    MenuUpdate(MenuContext),
}

pub struct MenuInit {
    fetch_state: FetchState<api::Menu>,
    api_context: ApiContext,
    menu_context: MenuContext,
}

impl Component for MenuInit {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (api_context, _) = ctx
            .link()
            .context::<ApiContext>(ctx.link().callback(Msg::ApiUpdate))
            .expect("context should exist");
        let (menu_context, _) = ctx
            .link()
            .context::<MenuContext>(ctx.link().callback(Msg::MenuUpdate))
            .expect("context should exist");
        Self {
            fetch_state: FetchState::NotFetching,
            api_context,
            menu_context,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                match &self.fetch_state {
                    FetchState::Success(menu) => {
                        self.menu_context.dispatch(MenuAction::Set(api::MenuCreate {
                            recipes: menu.recipes.clone(),
                            ingredients: menu.ingredients.clone(),
                        }));
                    }
                    _ => {}
                }
                true
            }
            Msg::Fetch => {
                let api = self.api_context.api();
                ctx.link().send_future(async move {
                    match api.fetch_my_menu().await {
                        Ok(menu) => Msg::SetFetchState(FetchState::Success(menu)),
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
            Msg::MenuUpdate(menu_context) => {
                self.menu_context = menu_context;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let FetchState::NotFetching = &self.fetch_state {
            ctx.link().send_message(Msg::Fetch);
        }
        html! {}
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct MenuProviderProps {
    pub children: Children,
}

#[function_component]
pub fn MenuProvider(props: &MenuProviderProps) -> Html {
    let menu = use_reducer(|| MenuHandle::new());

    html! {
        <ContextProvider<MenuContext> context={menu}>
            <MenuInit/>
            {props.children.clone()}
        </ContextProvider<MenuContext>>
    }
}
