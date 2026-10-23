use leptos::*;
use neuropulse_wasm::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
