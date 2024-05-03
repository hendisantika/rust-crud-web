use yew::{Component, ComponentLink, ShouldRender};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

mod item;
mod modal;
mod input;

const KEY: &'static str = "yew.rust.crud.database";

pub struct Model {
    storage: StorageService,
    state: List,
    link: ComponentLink<Self>,
}