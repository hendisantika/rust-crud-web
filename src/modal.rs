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

pub enum ModalMsg {
    HideModal,
    SetName(String),
    SetPrice(String),
    Save,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProperties;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            item: prop.item,
            name: "".to_string(),
            price: "".to_string(),
            visible: prop.visible,
            on_close: prop.on_close,
            on_save: prop.on_save,
            error: None,
            link,
        }
    }
}