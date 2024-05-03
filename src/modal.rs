use yew::{Callback, Component, ComponentLink, Properties, ShouldRender};
use yew::services::ConsoleService;

use crate::item::Item;

#[derive(Properties, Clone)]
pub struct ModalProperties {
    pub item: Item,
    pub visible: bool,
    pub on_close: Callback<bool>,
    pub on_save: Callback<Item>,
}

pub struct Modal {
    pub item: Item,
    pub name: String,
    pub price: String,
    pub visible: bool,
    pub on_close: Callback<bool>,
    pub on_save: Callback<Item>,
    error: Option<Vec<ItemValidationErr>>,
    link: ComponentLink<Self>,
}