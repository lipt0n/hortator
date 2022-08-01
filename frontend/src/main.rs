use console_error_panic_hook::set_once as set_panic_hook;
use gloo_console::log;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    value: i64, // This will store the counter value
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>

                <div class="flex flex-col h-screen">
                    // A button to send the Increment message
                    <button class="rounded bg-green-400 text-white p-6" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>
                    <p class="rounded bg-white-400 text-black p-6">
                    { self.value }
                </p>

                    // A button to send the Decrement message
                    <button class="rounded bg-green-400 text-white p-6" onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>



                </div>

                // Display the current value of the counter


                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    set_panic_hook();
    yew::start_app::<Model>();
}
