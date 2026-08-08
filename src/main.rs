mod input;
mod item;
mod modal;
mod model;

use model::Model;

fn main() {
    yew::Renderer::<Model>::new().render();
}
