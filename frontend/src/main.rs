use frontend::components::basic_button::BasicButton;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <BasicButton />
        </div>
    }
}
