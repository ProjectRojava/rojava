use yew::prelude::*;

pub struct BasicButton;

pub enum Msg {}

impl Component for BasicButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1> {"basic_button"} </h1>
        }
    }
}
