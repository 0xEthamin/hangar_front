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
    // Le logger est toujours utile pour le débogage.
    wasm_logger::init(wasm_logger::Config::default());
    
    // On lance directement l'application. La gestion de la langue se fera dans le composant App.
    yew::Renderer::<App>::new().render();
}