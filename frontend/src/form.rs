use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::html::IntoPropValue;

use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew::functional::function_component as fc;
use yew::{
    html, use_effect, use_reducer, Callback, Children, ContextProvider, Event, Html, Properties,
    Reducible,
};
//use serde_json;
//use serde::Deserialize;
use web_sys::{EventTarget, FocusEvent, HtmlInputElement, KeyboardEvent};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]

pub struct Model {
    pub form: HashMap<String, String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct LoginAction {
    pub key: String,
    pub value: String,
    pub form_id: String,
}

impl Reducible for Model {
    type Action = LoginAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_model = &*self.clone();
        let mut new_model = new_model.clone();
        //log!(serde_json::to_string(&action.clone()).unwrap());
        new_model
            .form
            .insert(action.key.clone(), action.value.clone());

        Rc::new(new_model)
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct FormProps {
    pub form_id: String,
    pub children: Children,
    pub debug: Option<bool>,
}

#[fc(Form)]
pub fn form(props: &FormProps) -> Html {
    let model = use_reducer(|| Model::default());

    let model_ = model.clone();
    if props.debug.is_some() && props.debug.unwrap() {
        use_effect(move || {
            let model = model_.clone();
            log!("model changed ", serde_json::to_string(&*model).unwrap());
            || log!("")
        });
    }

    let onsubmit = |e: FocusEvent| {
        e.prevent_default();
        let event: FocusEvent = e.clone();
        let target = &event.target().unwrap();
        log!("submit", target);
    };
    let handle_submit = Callback::from(onsubmit);

    let model_ = model.clone();
    let form_id = props.form_id.clone();
    let onkeypress = {
        Callback::from(move |event: KeyboardEvent| {
            let target: EventTarget = (&event.target().unwrap()).clone();
            let html_target = target.dyn_into::<HtmlInputElement>().unwrap();
            let name = html_target.clone().name();
            let value = html_target.clone().value();
            let action = LoginAction {
                key: name,
                value: value,
                form_id: form_id.clone(),
            };
            model_.dispatch(action)
        })
    };

    let model_ = model.clone();
    let form_id = props.form_id.clone();
    let onchange = {
        Callback::from(move |event: Event| {
            let target: EventTarget = event.target().unwrap().clone();
            let html_target = target.dyn_into::<HtmlInputElement>().unwrap();
            let name = html_target.clone().name();
            let value = html_target.clone().value();
            let action = LoginAction {
                key: name,
                value: value,
                form_id: form_id.clone(),
            };
            model_.dispatch(action)
        })
    };
    let model = model.clone();
    let model_clone = (*model).clone();

    html! {       <ContextProvider<Model> context={model_clone}>
                         <form
                         class="mt-8 space-y-6"
                         name={props.form_id.clone()}
                         id={props.form_id.clone()}
                         onchange={onchange}
                         onkeypress={onkeypress}
                         onsubmit={handle_submit}
                         >{for props.children.iter()}
                         </form>
                     </ContextProvider<Model>>
    }
}
