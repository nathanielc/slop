use patternfly_yew::prelude::{
    Cell, CellContext, Content, List, Spinner, Stack, StackItem, Table, TableColumn,
    TableEntryRenderer, TableHeader, TableModel, TableModelEntry,
};
use yew::{html, html_nested, Component, Context, Html};
use yew_nested_router::components::Link;

use crate::{
    api::{self, FetchState},
    api_context::ApiContext,
    app::Route,
    menu_context::MenuContext,
    recipe_link::RecipeLink,
};

#[derive(Debug)]
pub enum Msg {
    SetFetchState(FetchState<api::Menu>),
    Fetch,
    ApiUpdate(ApiContext),
    MenuUpdate(MenuContext),
}

pub struct Menu {
    fetch_state: FetchState<api::Menu>,
    api_context: ApiContext,
    menu_context: MenuContext,
}
impl Component for Menu {
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
        let api = self.api_context.api();
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            Msg::Fetch => {
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
        match &self.fetch_state {
            FetchState::NotFetching => {
                ctx.link().send_message(Msg::Fetch);
                html! {
                        <Spinner/>
                }
            }
            FetchState::Fetching => html! { <Spinner/> },
            FetchState::Success(data) => {
                let recipes = if data.recipes.is_empty() {
                    let book = html! {<Link<Route> target={Route::Book}>{"book"}</Link<Route>>};
                    let my_recipes =
                        html! {<Link<Route> target={Route::MyRecipes}>{"here"}</Link<Route>>};
                    let discover =
                        html! {<Link<Route> target={Route::Discover}>{"here"}</Link<Route>>};
                    html! {
                        <Content>
                            <p>
                                {"You have no recipes in your current menu. You can add recipes from your "}
                                {book}
                                {". Create new recipes "}
                                {my_recipes}
                                {" or find new recipes "}
                                {discover}
                                {"."}
                            </p>
                        </Content>
                    }
                } else {
                    html! {
                        <List>
                        {for data.recipes.iter().map(|recipe| {
                            html! {
                                <RecipeLink id={recipe.id.clone()}>{recipe.title.clone()}</RecipeLink>
                            }
                        })}
                        </List>
                    }
                };
                let header = html_nested! {
                    <TableHeader<Column>>
                        <TableColumn<Column> index={Column::Check} />
                        <TableColumn<Column> index={Column::Name} label="Ingredient" />
                        <TableColumn<Column> index={Column::Amount} label="Amount" />
                    </TableHeader<Column>>
                };
                let mut entries = TableData {
                    entries: data
                        .ingredients
                        .iter()
                        .map(|ingredient| Ingredient {
                            name: ingredient.name.clone(),
                            amount: ingredient.amount.clone(),
                        })
                        .collect(),
                };
                entries.entries.sort_by(|a, b| a.name.cmp(&b.name));
                html! {
                    <Stack gutter=true>
                        <StackItem><Content><h2>{"Recipes"}</h2></Content></StackItem>
                        <StackItem>{recipes}</StackItem>
                        <StackItem><Content><h2>{"Shopping List"}</h2></Content></StackItem>
                        <StackItem><Table<Column,TableData<Ingredient>> {header} {entries} /></StackItem>
                    </Stack>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Column {
    Check,
    Name,
    Amount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ingredient {
    name: String,
    amount: String,
}

impl TableEntryRenderer<Column> for Ingredient {
    /// Render the cell for the requested column.
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            // Use noop checkbox to make it easy to check off items while shopping
            Column::Check => html! {
                <input type={"checkbox"}/>
            },
            Column::Name => html!(&self.name),
            Column::Amount => html!(&self.amount),
        }
        .into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableData<T> {
    entries: Vec<T>,
}

impl<C, T> TableModel<C> for TableData<T>
where
    C: Clone + Eq + 'static,
    T: TableEntryRenderer<C> + 'static,
{
    type Iterator<'i> = Box<dyn Iterator<Item = TableModelEntry<'i, T, usize>> + 'i>;

    type Item = T;

    type Key = usize;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn iter(&self) -> Self::Iterator<'_> {
        Box::new(
            self.entries
                .iter()
                .enumerate()
                .map(|(i, entry)| TableModelEntry {
                    key: i,
                    value: entry,
                    expanded: true,
                }),
        )
    }
}
