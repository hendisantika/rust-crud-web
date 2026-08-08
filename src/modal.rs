use yew::{html, Callback, Component, Context, Html, MouseEvent, Properties, SubmitEvent};

use crate::input::TextInput;
use crate::item::{Item, ItemFormData, ItemValidationErr};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    /// The record being edited, or `Item::default()` when creating a new one.
    pub item: Item,
    pub on_close: Callback<()>,
    pub on_save: Callback<Item>,
}

/// The create/update dialog. It is mounted only while it is open, so its draft
/// state always starts from the item handed over by the parent.
pub struct Modal {
    name: String,
    price: String,
    errors: Vec<ItemValidationErr>,
}

pub enum ModalMsg {
    Close,
    SetName(String),
    SetPrice(String),
    Save,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::draft_from(&ctx.props().item)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // The parent hands over fresh callbacks on every render, so props are never
        // equal by value. Only a genuinely different record may discard the draft.
        if old_props.item != ctx.props().item {
            *self = Self::draft_from(&ctx.props().item);
        }

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMsg::Close => {
                ctx.props().on_close.emit(());
                false
            }

            ModalMsg::SetName(name) => {
                self.name = name;
                true
            }

            ModalMsg::SetPrice(price) => {
                self.price = price;
                true
            }

            ModalMsg::Save => {
                let form_data = ItemFormData::from((self.name.clone(), self.price.clone()));

                match form_data.validate() {
                    Ok(valid) => {
                        self.errors.clear();
                        ctx.props().on_save.emit(Item {
                            id: ctx.props().item.id,
                            name: valid.name,
                            price: valid.price,
                        });
                    }

                    Err(errors) => self.errors = errors,
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let close = link.callback(|_: MouseEvent| ModalMsg::Close);
        let onsubmit = link.callback(|e: SubmitEvent| {
            e.prevent_default();
            ModalMsg::Save
        });

        let title = if ctx.props().item.is_new() {
            "New Item"
        } else {
            "Update Item"
        };

        html! {
            <div class="modal is-active">
                <div class="modal-background" onclick={close.clone()}></div>
                <div class="modal-card">
                    <form {onsubmit}>
                        <header class="modal-card-head">
                            <p class="modal-card-title">{title}</p>
                            <button
                                type="button"
                                onclick={close.clone()}
                                class="delete"
                                aria-label="close"
                            />
                        </header>
                        <section class="modal-card-body">
                            {self.view_errors()}
                            <div class="field">
                                <label class="label">{"Name"}</label>
                                <div class="control">
                                    <TextInput
                                        value={self.name.clone()}
                                        placeholder="e.g. Mechanical Keyboard"
                                        autofocus=true
                                        oninput={link.callback(ModalMsg::SetName)}
                                    />
                                </div>
                            </div>

                            <div class="field">
                                <label class="label">{"Price"}</label>
                                <div class="control has-icons-left">
                                    <TextInput
                                        value={self.price.clone()}
                                        placeholder="e.g. 49.90"
                                        oninput={link.callback(ModalMsg::SetPrice)}
                                    />
                                    <span class="icon is-small is-left">
                                        <i class="icon ion-md-cash"></i>
                                    </span>
                                </div>
                            </div>
                        </section>
                        <footer class="modal-card-foot">
                            <button type="submit" class="button is-info">{"Save"}</button>
                            <button type="button" onclick={close} class="button">{"Cancel"}</button>
                        </footer>
                    </form>
                </div>
            </div>
        }
    }
}

impl Modal {
    fn draft_from(item: &Item) -> Self {
        Self {
            name: item.name.clone(),
            // A brand new item starts with an empty price field rather than "0".
            price: if item.is_new() {
                String::new()
            } else {
                item.formatted_price()
            },
            errors: Vec::new(),
        }
    }

    fn view_errors(&self) -> Html {
        if self.errors.is_empty() {
            return Html::default();
        }

        html! {
            <div class="notification is-danger">
                {for self.errors.iter().map(|e| html! { <div>{e.message()}</div> })}
            </div>
        }
    }
}
