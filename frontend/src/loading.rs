use yew::functional::function_component as fc;
use yew::{html, use_state, use_effect, use_effect_with_deps, Callback};

#[fc(Loading)]
pub fn list(text:Option(&[str]))->Html {

    html!(<div></div>)
}