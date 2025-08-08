use leptos::mount::mount_to_body;
use leptos::prelude::*;
use std::env;

const BUILD_VERSION: &str = env!("BUILD_SEMVER");
const BUILD_COUNT: &str = env!("BUILD_COUNT");

fn main() {
    console_error_panic_hook::set_once();

    println!("oai-client");
    println!("v{} (Build {})", BUILD_VERSION, BUILD_COUNT);
    println!("---");

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (text, set_text) = signal(String::new());

    view! {
        <h1>OAI Client</h1>
        <div>
            <dl>
                <dt>Build Version:</dt>
                <dd>{BUILD_VERSION}</dd>
                <dt>Build Count:</dt>
                <dd>{BUILD_COUNT}</dd>
            </ dl>
        </div>
        <textarea
            on:input:target=move |evt| set_text.set(evt.target().value())
            prop:value=move || text.get()
        >{text}</textarea>
        <br />
        <button
            on:click=move |_| {
                *set_count.write() += 1 // increment on click
            }

            // Apply red color if the count is odd
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
        <div>
            {move || text.get()}
        </div>
    }
}
