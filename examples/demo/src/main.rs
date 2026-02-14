use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Ethereum Boilerplate Demo"</h1>
            <p>"Demo application is working!"</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
