use yew::{function_component, html};
// create a yew component using bulma.css that renders a file dropdown menu
#[function_component(FileMenu)]
pub fn file_menu() -> yew::Html {
    html! {
        <div class="dropdown is-hoverable">
            <div class="dropdown-trigger">
                <button class="button" aria-haspopup="true" aria-controls="dropdown-menu4">
                    <span>{"File"}</span>
                    <span class="icon is-small">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                </button>
            </div>
            <div class="dropdown-menu" id="dropdown-menu4" role="menu">
                <div class="dropdown-content">
                    <a class="dropdown-item">
                        <span>{"New"}</span>
                    </a>
                    <a class="dropdown-item">
                        <span>{"Open"}</span>
                    </a>
                    <hr class="dropdown-divider"/>
                    <a class="dropdown-item">
                        <span>{"Save"}</span>
                    </a>
                    <a class="dropdown-item">
                        <span>{"Save As"}</span>
                    </a>
                    <hr class="dropdown-divider"/>
                    <a class="dropdown-item">
                        <span>{"Close"}</span>
                    </a>
                </div>
            </div>
        </div>
    }
}
