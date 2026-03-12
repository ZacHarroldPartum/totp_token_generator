use dioxus::prelude::*;

use components::Totp;

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut hash = use_signal(|| "".to_string());

    use_context_provider(|| AppContext { hash });

    spawn(async move {
        let value = document::eval(r#"
            const val = window.location.hash;
            return val;
        "#).await.as_ref().map(ToString::to_string).unwrap_or_default();
        let value = value.strip_prefix("\"#").unwrap_or_default().strip_suffix('"').unwrap_or_default().to_string();
        hash.set(value);
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Totp {}
    }
}

#[derive(Clone, Copy)]
struct AppContext {
    hash: Signal<String>,
}
