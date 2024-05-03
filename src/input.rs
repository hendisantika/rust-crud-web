use yew::{Callback, Component, ComponentLink, InputData, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct TextInputProps {
    pub value: String,
    pub oninput: Callback<String>,
}

pub struct TextInput {
    value: String,
    link: ComponentLink<Self>,
    oninput: Callback<String>,
}

pub enum TextInputMsg {
    Changed(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput {
            value: props.value,
            oninput: props.oninput,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextInputMsg::Changed(value) => {
                self.oninput.emit(value);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.oninput = props.oninput;
        true
    }
}