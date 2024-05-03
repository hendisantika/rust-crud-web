use yew::{Component, ComponentLink, ShouldRender};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

use crate::item::Item;

mod item;
mod modal;
mod input;

const KEY: &'static str = "yew.rust.crud.database";

pub struct Model {
    storage: StorageService,
    state: List,
    link: ComponentLink<Self>,
}

pub struct List {
    items: Vec<Item>,
    modal_visible: bool,
    current_item: Option<Item>,
}

pub enum Msg {
    New,
    HiddenModal,
    Saved(Item),
    Edit(usize),
    Remove(usize),
    Store,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Storage Error");

        let items = {
            if let Json(Ok(items)) = storage.restore(KEY) {
                items
            } else {
                Vec::new()
            }
        };

        let state = List {
            items,
            modal_visible: false,
            current_item: None,
        };

        Model { storage, state, link }
    }
}