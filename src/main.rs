mod app;
mod components;
mod contexts;
mod models;
mod pages;
mod router;
mod services;

use app::App;

fn main() 
{
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}