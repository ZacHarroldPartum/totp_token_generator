use core::time::Duration;
use dioxus::prelude::*;
use dioxus_sdk_time::use_interval;
use totp_rs::{Algorithm, Secret, TOTP};

#[component]
pub fn Totp() -> Element {
    let context = use_context::<crate::AppContext>();

    let mut digits = use_signal(|| 6);
    let mut step = use_signal(|| 30);
    let mut now = use_signal(|| 0);
    let mut secret = context.hash;

    use_interval(Duration::from_secs(1), move |()| {
        if let Ok(value) = u64::try_from(jiff::Timestamp::now().as_second()) {
            now.set(value);
        }
    });

    let totp = use_memo(move || {
        let secret = Secret::Encoded(secret().to_ascii_uppercase());
        let bytes = secret.to_bytes().ok()?;
        TOTP::new(Algorithm::SHA1, digits(), 1, step(), bytes).ok()
    });

    let token = use_memo(move || {
        Some(totp()?.generate(now()))
    });

    rsx! {
        div {
            class: "mx-auto flex max-w-sm items-center gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10",

            div {
                div {
                    class: "text-4xl font-medium text-black dark:text-white text-center mb-4",
                    "{token().unwrap_or(\"Enter Token Parameters\".to_string())}"
                }

                progress {
                    width: "100%",
                    max: "{step()}",
                    value: "{now() % step()}"
                }

                div {
                    class: "grid gap-6 mb-6 md:grid-cols-2",
                    div {
                        label { for: "digits", class: "block mb-2.5 text-sm font-medium text-heading", "Digits" }
                        input {
                            name: "digits",
                            r#type: "number",
                            class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                            min: 1,
                            step: 1,
                            value: "{digits()}",
                            oninput: move |event| digits.set(event.value().parse().unwrap_or(6))
                        }
                    }
                    div {
                        label { for: "step", class: "block mb-2.5 text-sm font-medium text-heading", "Step" }
                        input {
                            name: "step",
                            r#type: "number",
                            class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                            min: 1,
                            step: 1,
                            value: "{step()}",
                            oninput: move |event| step.set(event.value().parse().unwrap_or(30))
                        }
                    }
                    div {
                        class: "col-span-2",
                        label { for: "secret", class: "block mb-2.5 text-sm font-medium text-heading", "Secret" }
                        input {
                            name: "secret",
                            r#type: "password",
                            class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                            value: "{secret()}",
                            oninput: move |event| secret.set(event.value())
                        }
                    }
                }
            }
        }
    }
}
