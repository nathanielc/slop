mod add_to_book;
mod api;
mod api_context;
mod app;
mod book;
mod discover;
mod edit_recipe;
mod menu;
mod menu_context;
mod my_recipes;
mod new_recipe;
mod pagination;
mod recipe;
mod recipe_link;
mod settings;
mod slop;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<app::Application>::new().render();
}
