use yew::{Callback, Component, ComponentLink, InputData, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct TextInputProps {
    pub value: String,
    pub oninput: Callback<String>,
}