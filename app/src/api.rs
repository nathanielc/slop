use gloo::utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/api/dist/api.js")]
extern "C" {
    #[derive(Clone, Debug, PartialEq)]
    type Api;

    #[wasm_bindgen(constructor)]
    pub fn new(address: JsValue) -> Api;

    #[wasm_bindgen(method, catch)]
    async fn is_authenticated(this: &Api) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn authenticate(this: &Api) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_my_menu(this: &Api) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_book_tags(this: &Api) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_book_entries(this: &Api, tag: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_recipe(this: &Api, id: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_all_recipes(this: &Api, page: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn fetch_my_recipes(this: &Api, page: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn create_menu(this: &Api, menu_create: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn create_recipe(this: &Api, recipe_create: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn update_recipe(
        this: &Api,
        id: JsValue,
        recipe_update: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn create_book_entry(this: &Api, book_entry_create: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn update_book_entry(
        this: &Api,
        id: JsValue,
        book_entry_update: JsValue,
    ) -> Result<JsValue, JsValue>;
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}
impl From<serde_json::Error> for FetchError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            err: JsValue::from_str(&e.to_string()),
        }
    }
}

/// The possible states a fetch request can be in.
#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct BookEntries {
    pub tag: String,
    pub entries: Vec<BookEntry>,
}

impl TryFrom<JsValue> for BookEntries {
    type Error = FetchError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(value.into_serde()?)
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct BookEntry {
    pub id: String,
    #[serde(rename = "recipeId")]
    pub recipe_id: String,
    pub recipe: Option<String>,
    pub title: String,
    pub tag: String,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
struct BookEntryCreate {
    #[serde(rename = "recipeId")]
    pub recipe_id: String,
    pub title: String,
    pub tag: String,
}

#[derive(Serialize, PartialEq, Debug, Default, Clone)]
pub struct BookEntryUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "recipeId", skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub source: String,
    pub author: Author,
    pub deleted: bool,
}

impl TryFrom<JsValue> for Recipe {
    type Error = FetchError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(value.into_serde()?)
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Recipes {
    pub recipes: Vec<Recipe>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

impl TryFrom<JsValue> for Recipes {
    type Error = FetchError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(value.into_serde()?)
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
struct RecipeCreate {
    pub source: String,
}

#[derive(Serialize, PartialEq, Debug, Default, Clone)]
pub struct RecipeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MenuRecipe {
    pub id: String,
    pub title: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MenuIngredient {
    pub name: String,
    pub amount: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Menu {
    pub id: String,
    #[serde(default)]
    pub recipes: Vec<MenuRecipe>,
    #[serde(default)]
    pub ingredients: Vec<MenuIngredient>,
}

impl TryFrom<JsValue> for Menu {
    type Error = FetchError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(value.into_serde()?)
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct MenuCreate {
    pub recipes: Vec<MenuRecipe>,
    pub ingredients: Vec<MenuIngredient>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Forward,
    Backward,
}
#[derive(Serialize, PartialEq, Debug, Default, Clone)]
pub struct Page {
    #[serde(skip_serializing)]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<usize>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct PageInfo {
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Author {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApiHandle {
    api: Api,
    whoami: RefCell<Option<Author>>,
}

impl ApiHandle {
    pub fn new(address: String) -> Self {
        Self {
            api: Api::new(JsValue::from(address)),
            whoami: RefCell::new(None),
        }
    }
    pub fn whoami(&self) -> impl std::ops::Deref<Target = Option<Author>> + '_ {
        self.whoami.borrow()
    }
    async fn ensure_authenticated(&self) -> Result<(), FetchError> {
        if !self.is_authenticated().await? {
            let author = self.authenticate().await?;
            *self.whoami.borrow_mut() = Some(author);
        }
        Ok(())
    }
    pub async fn is_authenticated(&self) -> Result<bool, FetchError> {
        let res = self.api.is_authenticated().await?;
        let result: bool = res.into_serde()?;
        Ok(result)
    }
    pub async fn authenticate(&self) -> Result<Author, FetchError> {
        let res = self.api.authenticate().await?;
        let result: Author = res.into_serde()?;
        Ok(result)
    }
    pub async fn fetch_my_menu(&self) -> Result<Menu, FetchError> {
        self.ensure_authenticated().await?;
        let value = self.api.fetch_my_menu().await?;
        Ok(value.into_serde()?)
    }
    pub async fn fetch_book_tags(&self) -> Result<Vec<String>, FetchError> {
        self.ensure_authenticated().await?;
        let value = self.api.fetch_book_tags().await?;
        Ok(value.into_serde()?)
    }
    pub async fn fetch_book_entries(&self, tag: String) -> Result<BookEntries, FetchError> {
        self.ensure_authenticated().await?;
        let value = self
            .api
            .fetch_book_entries(JsValue::from(tag.clone()))
            .await?;
        value.try_into()
    }
    pub async fn fetch_recipe(&self, id: String) -> Result<Recipe, FetchError> {
        let value = self.api.fetch_recipe(JsValue::from(id)).await?;
        value.try_into()
    }
    pub async fn fetch_all_recipes(&self, page: &Page) -> Result<Recipes, FetchError> {
        let value = self
            .api
            .fetch_all_recipes(JsValue::from_serde(page)?)
            .await?;
        value.try_into()
    }
    pub async fn fetch_my_recipes(&self, page: &Page) -> Result<Recipes, FetchError> {
        self.ensure_authenticated().await?;
        let value = self
            .api
            .fetch_my_recipes(JsValue::from_serde(page)?)
            .await?;
        value.try_into()
    }

    pub async fn create_menu(&self, create: MenuCreate) -> Result<(), FetchError> {
        self.ensure_authenticated().await?;
        let js_create = JsValue::from_serde(&create)?;
        let _res = self.api.create_menu(js_create).await?;
        Ok(())
    }

    pub async fn create_recipe(&self, source: String) -> Result<String, FetchError> {
        self.ensure_authenticated().await?;
        let create = RecipeCreate { source };
        let js_create = JsValue::from_serde(&create)?;
        let id = self.api.create_recipe(js_create).await?;
        Ok(id.into_serde()?)
    }

    pub async fn update_recipe(&self, id: &str, update: &RecipeUpdate) -> Result<(), FetchError> {
        self.ensure_authenticated().await?;
        let _res = self
            .api
            .update_recipe(JsValue::from(id), JsValue::from_serde(update)?)
            .await?;
        Ok(())
    }

    pub async fn create_book_entry(
        &self,
        recipe_id: String,
        title: String,
        tag: String,
    ) -> Result<(), FetchError> {
        self.ensure_authenticated().await?;
        let create = BookEntryCreate {
            recipe_id,
            title,
            tag,
        };
        let js_create = JsValue::from_serde(&create)?;
        let _res = self.api.create_book_entry(js_create).await?;
        Ok(())
    }
    pub async fn update_book_entry(
        &self,
        id: &str,
        update: &BookEntryUpdate,
    ) -> Result<(), FetchError> {
        self.ensure_authenticated().await?;
        let _res = self
            .api
            .update_book_entry(JsValue::from(id), JsValue::from_serde(update)?)
            .await?;
        Ok(())
    }
}
