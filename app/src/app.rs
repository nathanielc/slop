use patternfly_yew::prelude::{
    BackdropViewer, Nav, NavList, NavRouterItem, Page, PageSidebar, ToastViewer,
};
use yew::functional::*;
use yew::prelude::*;
use yew_nested_router::prelude::*;

use crate::{
    api_context::ApiProvider, book::Book, discover::Discover, edit_recipe::EditRecipe, menu::Menu,
    menu_context::MenuProvider, my_recipes::MyRecipes, new_recipe::NewRecipe, recipe::Recipe,
    settings::Settings,
};

#[derive(Clone, Debug, PartialEq, Eq, Target)]
pub enum Route {
    #[target(index)]
    Menu,
    NewRecipe,
    Recipe {
        id: String,
    },
    EditRecipe {
        id: String,
    },
    Discover,
    MyRecipes,
    Book,
    Settings,
    NotFound,
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <ApiProvider>
            <MenuProvider>
                <BackdropViewer>
                    <ToastViewer>
                        <Router<Route>>
                            <Switch<Route> render={switch}/>
                        </Router<Route>>
                    </ToastViewer>
                </BackdropViewer>
            </MenuProvider>
        </ApiProvider>
    }
}

fn switch(target: Route) -> Html {
    match target {
        Route::Menu => html!(<AppPage><Menu /></AppPage>),
        Route::NewRecipe => html!(<AppPage><NewRecipe /></AppPage>),
        Route::Recipe { id } => html!(<AppPage><Recipe id={id}/></AppPage>),
        Route::EditRecipe { id } => html!(<AppPage><EditRecipe id={id}/></AppPage>),
        Route::Discover => html!(<AppPage><Discover/></AppPage>),
        Route::MyRecipes => html!(<AppPage><MyRecipes/></AppPage>),
        Route::Book => html!(<AppPage><Book/></AppPage>),
        Route::Settings => html!(<AppPage><Settings/></AppPage>),
        Route::NotFound => html!(<AppPage><h1>{ "404" }</h1></AppPage>),
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavRouterItem<Route> to={Route::Menu}>{"Menu"}</NavRouterItem<Route>>
                    <NavRouterItem<Route> to={Route::Book}>{"Recipe Book"}</NavRouterItem<Route>>
                    <NavRouterItem<Route> to={Route::Discover}>{"Discover Recipes"}</NavRouterItem<Route>>
                    <NavRouterItem<Route> to={Route::MyRecipes}>{"My Recipes"}</NavRouterItem<Route>>
                    <NavRouterItem<Route> to={Route::Settings}>{"Settings"}</NavRouterItem<Route>>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    html!(
        <Page sidebar={sidebar}>
            <div class="slop-page">
                { for props.children.iter() }
            </div>
        </Page>
    )
}
