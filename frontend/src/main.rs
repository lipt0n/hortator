use console_error_panic_hook::set_once as set_panic_hook;
use gloo_console::log;
use yew::functional::function_component as fc;
use yew::{html, use_effect_with_deps};
mod form;
mod login;
// Define the possible messages which can be sent to the component

#[fc(Main)]
fn main_component() -> Html {
    use_effect_with_deps(
        move |_| {
            log!("1 CounterComponent updated");
            || log!("1 hook cleanup")
        },
        0, // deps works as second param in react
    );

    html! {
        <div>
        <login::Login />

        </div>
    }
}

fn main() {
    set_panic_hook();

    yew::start_app::<Main>();
}
