use leptos::mount::mount_to_body;
use leptos::*;
use leptos_osuapi_playground::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    });
}
