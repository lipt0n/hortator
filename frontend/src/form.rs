use gloo::net::http::Request;
use gloo_console::{log, table};
use reqwest_wasm::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventTarget, FocusEvent, HtmlInputElement, KeyboardEvent};
use yew::functional::function_component as fc;
use yew::{
    html, use_effect, use_reducer, Callback, Children, ContextProvider, Event, Properties,
    Reducible,
};
use yew_hooks::use_async;

#[derive(Clone, Debug, PartialEq)]

enum Error {
    RequestError,
    DeserializeError,
}

async fn post(url: &str, params: &HashMap<String, String>) -> Result<String, Error> {
    log!("# POST CALLED from url: {}", url);
    log!("# POST PARAMS:");
    let t = params.values().cloned().collect::<Vec<_>>();
    //let v = JsValue::from_serde(&t).unwrap();

    //   table!(JsValue::from_serde(&t).unwrap());
    //  table!(JsValue::from_serde(params).unwrap());
    //let response = Client::new().post(url).json(params).send().await;
    let response = Request::post(url).json(&t).send().await;
    log!("# POST RESPONSE!");
    // if let Ok(data) = response {
    //     match data.json::<serde_json::Value>().await {
    //         Ok(data) => {
    //             log!("# POST RESPONSE DATA:");
    //             //    table!(data);
    //             Ok(data)
    //         }
    //         Err(e) => Err(Error::DeserializeError),
    //     }
    // } else {
    //     Err(Error::RequestError)
    // }
    if let Ok(data) = response {
        if let Ok(repo) = data.text().await {
            log!("# POST RESPONSE DATA: ok");
            Ok(repo)
        } else {
            log!("# POST RESPONSE DATA: error 0 ");
            Err(Error::DeserializeError)
        }
    } else {
        log!("# POST RESPONSE DATA: error 0 ");

        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]

pub struct Model {
    pub form: HashMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    pub target: String,
    pub debug: Option<bool>,
    pub class: Option<String>,
}

#[fc(Form)]
pub fn form(props: &FormProps) -> Html {
    let model = use_reducer(|| Model::default());
    let target = props.clone().target.to_string();

    let model_ = model.clone();
    let handler = use_async(async move {
        let form = &model_.clone().form;
        log!("# HANDLER BEFORE CALLING POST");
        post(&target, form).await
    });

    let model_ = model.clone();
    if props.debug.is_some() && props.debug.unwrap() {
        use_effect(move || {
            let model = model_.clone();
            table!(JsValue::from_serde(&*model).unwrap());

            || log!("")
        });
    }

    let handler_ = handler.clone();
    let onsubmit = move |e: FocusEvent| {
        e.prevent_default();
        let event: FocusEvent = e.clone();
        let target = &event.target().unwrap();
        log!("# SUMBIT CALLED", target);
        handler_.run()
    };
    //let handle_submit = Callback::from(onsubmit);

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
                            class={props.class.clone()}
                            name={props.form_id.clone()}
                            id={props.form_id.clone()}
                            onchange={onchange}
                            onkeypress={onkeypress}
                            onsubmit={onsubmit}
                            >
                                if handler.loading{ <h1>{"LOADING .... waiting for server response"}</h1>}
                                {for props.children.iter()}
                        </form>
                     </ContextProvider<Model>>
    }
}
