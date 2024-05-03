// use crate::model::Model;
//
// mod input;
// mod item;
// mod modal;
// mod model;
//mod document;
use yew_dev_viewer::Model;

fn main() {
    println!("Hello, world!");
    // yew::start_app::<Model>();
    yew::Renderer::<Model>::new().render();
}
