// create a functional component for a text area for text editor using yew components and material-yew crate
use material_yew::text_inputs::TextFieldType;
use material_yew::MatTextArea;
use yew::{function_component, html};

#[function_component(TextEditor)]
pub fn text_editor() -> yew::Html {
    html! {
        <MatTextArea outlined=true field_type={TextFieldType::Text} rows=50 cols=300  />
    }
}
