use material_yew::MatButton;
use rojava_frontend::components::text_editor::TextEditor;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <MatButton label="Click me!" />
            <TextEditor />
        </>
    }
}
