use yew::{function_component, html};
use yew_lucide::Camera;

#[function_component(App)]
fn app() -> Html {
    html! { <Camera /> }
}

fn main() {
    yew::start_app::<App>();
}
