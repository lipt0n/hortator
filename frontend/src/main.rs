use console_error_panic_hook::set_once as set_panic_hook;
use gloo_console::log;
use yew::functional::function_component as fc;
use yew::{html, use_state, use_effect, use_effect_with_deps, Renderer, Html};
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
  //  yew::renderer::App::<Main>::new().mount_to_body();
 //   yew::Renderer::<Main>::new().render();
    Renderer::<Main>::new().render();

   // yew::start_app::<Main>();
}
