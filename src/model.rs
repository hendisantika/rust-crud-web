use gloo_storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, MouseEvent};

use crate::input::TextInput;
use crate::item::Item;
use crate::modal::Modal;

const KEY: &str = "yew.rust.crud.database";

/// The root component: it owns the item list and is the only place that talks
/// to `localStorage`.
pub struct Model {
    items: Vec<Item>,
    /// `Some(draft)` while the create/update dialog is open.
    editing: Option<Item>,
    search: String,
}

pub enum Msg {
    New,
    Edit(usize),
    Remove(usize),
    CloseModal,
    Saved(Item),
    Search(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            items: LocalStorage::get(KEY).unwrap_or_default(),
            editing: None,
            search: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::New => {
                self.editing = Some(Item::default());
                true
            }

            Msg::Edit(id) => {
                self.editing = self.find(id).cloned();
                self.editing.is_some()
            }

            Msg::Remove(id) => {
                self.items.retain(|item| item.id != id);
                self.store();
                true
            }

            Msg::CloseModal => {
                self.editing = None;
                true
            }

            Msg::Saved(item) => {
                match self
                    .items
                    .iter_mut()
                    .find(|existing| existing.id == item.id)
                {
                    Some(existing) => *existing = item,
                    None => {
                        self.items.push(Item {
                            id: self.next_id(),
                            ..item
                        });
                    }
                }

                self.editing = None;
                self.store();
                true
            }

            Msg::Search(search) => {
                self.search = search;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let modal = match self.editing.as_ref() {
            None => Html::default(),
            Some(item) => html! {
                <Modal
                    item={item.clone()}
                    on_close={link.callback(|_| Msg::CloseModal)}
                    on_save={link.callback(Msg::Saved)}
                />
            },
        };

        html! {
            <>
                {modal}
                <section class="hero is-small is-info is-bold">
                    <div class="hero-body">
                        <div class="container">
                            <p class="title">{"Items"}</p>
                            <p class="subtitle">{"List of items"}</p>
                        </div>
                    </div>
                </section>
                <main class="section">
                    <div class="container">
                        {self.view_toolbar(ctx)}
                        {self.view_table(ctx)}
                        {self.view_summary()}
                    </div>
                </main>
            </>
        }
    }
}

impl Model {
    fn find(&self, id: usize) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Ids are derived from the stored data so they survive a page reload.
    fn next_id(&self) -> usize {
        self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1
    }

    fn store(&self) {
        // Nothing useful can be done when the browser refuses to persist
        // (private mode, quota); the in-memory list stays authoritative.
        let _ = LocalStorage::set(KEY, &self.items);
    }

    fn visible_items(&self) -> Vec<&Item> {
        let needle = self.search.trim().to_lowercase();

        self.items
            .iter()
            .filter(|item| needle.is_empty() || item.name.to_lowercase().contains(&needle))
            .collect()
    }

    fn view_toolbar(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="level">
                <div class="level-left">
                    <div class="level-item">
                        <TextInput
                            value={self.search.clone()}
                            placeholder="Search by name"
                            oninput={link.callback(Msg::Search)}
                        />
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        <button
                            onclick={link.callback(|_: MouseEvent| Msg::New)}
                            type="button"
                            class="button is-info"
                        >
                            {"Add"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn view_table(&self, ctx: &Context<Self>) -> Html {
        let items = self.visible_items();

        let body = if items.is_empty() {
            html! {
                <tr>
                    <td colspan="5" class="has-text-centered has-text-grey">
                        {if self.items.is_empty() {
                            "No items yet. Use \"Add\" to create the first one."
                        } else {
                            "No item matches your search."
                        }}
                    </td>
                </tr>
            }
        } else {
            items
                .iter()
                .map(|item| self.view_item(ctx, item))
                .collect::<Html>()
        };

        html! {
            <table class="table is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th>{"Id"}</th>
                        <th>{"Name"}</th>
                        <th>{"Price"}</th>
                        <th colspan="2"></th>
                    </tr>
                </thead>
                <tbody>{body}</tbody>
            </table>
        }
    }

    fn view_item(&self, ctx: &Context<Self>, item: &Item) -> Html {
        let link = ctx.link();
        let id = item.id;

        html! {
            <tr key={id}>
                <td>{id}</td>
                <td>{&item.name}</td>
                <td>{item.formatted_price()}</td>
                <td>
                    <button
                        onclick={link.callback(move |_: MouseEvent| Msg::Edit(id))}
                        type="button"
                        class="button is-info is-outlined"
                    >
                        {"Edit"}
                    </button>
                </td>
                <td>
                    <button
                        onclick={link.callback(move |_: MouseEvent| Msg::Remove(id))}
                        type="button"
                        class="button is-danger is-outlined"
                    >
                        {"Remove"}
                    </button>
                </td>
            </tr>
        }
    }

    fn view_summary(&self) -> Html {
        let items = self.visible_items();

        if items.is_empty() {
            return Html::default();
        }

        let total: f64 = items.iter().map(|item| item.price).sum();

        html! {
            <p class="has-text-grey">
                {format!("{} item(s) — total {:.2}", items.len(), total)}
            </p>
        }
    }
}
