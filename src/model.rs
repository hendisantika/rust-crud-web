use yew::{Component, ComponentLink, Html, html, ShouldRender};
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::New => {
                self.state.modal_visible = true;
                self.state.current_item = None;

                true
            }

            Msg::HiddenModal => {
                self.state.modal_visible = false;
                true
            }

            Msg::Saved(item) => {
                if item.id == 0 {
                    let mut item = item;
                    item.id = Item::generate_id();
                    self.state.items.push(item);
                } else {
                    let index = self.state.items.iter().position(|i| i.id == item.id).unwrap();
                    self.state.items[index] = item;
                }

                self.update(Msg::HiddenModal);
                self.update(Msg::Store);

                true
            }

            Msg::Edit(idx) => {
                let item = self.state.items[idx].clone();
                self.state.current_item = Some(item);
                self.state.modal_visible = true;

                true
            }

            Msg::Remove(idx) => {
                self.state.items.remove(idx);
                self.update(Msg::Store);

                true
            }

            Msg::Store => {
                self.storage.store(KEY, Json(&self.state.items));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let modal = match self.state.current_item.as_ref() {
            None => {
                html! {
          <Modal: item=Item { ..Default::default() } visible=self.state.modal_visible on_close=self.link.callback(|_| { Msg::HiddedModal }) on_save=self.link.callback(Msg::Saved) />
        }
            }

            Some(item) => {
                html! {
          <Modal: item=item visible=self.state.modal_visible on_close=self.link.callback(|_| { Msg::HiddedModal }) on_save=self.link.callback(Msg::Saved) />
        }
            }
        };

        html! {
      <>
        {modal}
        <section class="hero is-small is-info is-bold">
          <div class="hero-body">
            <div class="container">
              <p class="title">
                {{ "Items" }}
              </p>
              <p class="subtitle">
                {{"List of items"}}
              </p>
            </div>
          </div>
        </section>
        <main class="section">
          <div class="container">
            {{self.view_table()}}
          </div>
        </main>
      </>
    }
    }
}

impl Model {
    fn view_table(&self) -> Html {
        html! {
      <>
        <table class="table is-hoverable is-fullwidth">
          <thead>
            <tr>
              <th>{"Id"}</th>
              <th>{"Name"}</th>
              <th>{"Price"}</th>
              <th colspan="2"></th>
            </tr>
          </thead>
          <tbody>
            {for self.state.items.iter().enumerate().map(|idx_itm| self.view_item(idx_itm))}
          </tbody>
        </table>

        <div>
          <button onclick=self.link.callback(|_| { Msg::New }) type="button" class="button is-info">{"Add"}</button>
        </div>
      </>
    }
    }
}