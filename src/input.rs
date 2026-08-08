use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Html, InputEvent, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value: String,
    /// Emits the current text on every keystroke.
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub autofocus: bool,
}

/// A thin, controlled wrapper around `<input class="input" />`.
#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let oninput = {
        let emit = props.oninput.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            emit.emit(input.value());
        })
    };

    html! {
        <input
            class="input"
            type="text"
            value={props.value.clone()}
            placeholder={props.placeholder.clone()}
            autofocus={props.autofocus}
            {oninput}
        />
    }
}
