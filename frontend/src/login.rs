use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::html::IntoPropValue;

use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew::functional::function_component as fc;
use yew::{
    html, use_effect, use_effect_with_deps, use_reducer, use_state, Callback, Event, Html,
    Reducible, SubmitEvent,
};
//use serde_json;
//use serde::Deserialize;
use web_sys::{EventTarget, HtmlInputElement, KeyboardEvent};

//use validator::{ Validate, ValidationError};
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}
#[derive(Clone, Debug)]
pub struct LoginAction {
    pub key: String,
    pub value: String,
}

impl Reducible for LoginModel {
    type Action = LoginAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_model = &*self.clone();
        let mut new_model = new_model.clone();
        match action.key.as_str() {
            "email" => new_model.email = action.value,
            "password" => new_model.password = action.value,
            _ => log!("Unknown key: {}", action.key),
        }
        Rc::new(new_model)
    }
}

// #[derive(Validate)]
// struct LoginModel {
//     #[validate("email")]
//     email: String,
//     #[validate(length(min = 10, message="password is required"))]
//     password: String,
//     #[validate(custom = "must_be_true")]
//     accept_terms: bool,
// }

#[fc(Login)]
pub fn login() -> Html {
    let model = use_reducer(|| LoginModel::default());

    let model_ = model.clone();
    use_effect(move || {
        let model = model_.clone();
        log!("model changed ", serde_json::to_string(&*model).unwrap());
        || log!("")
    });

    let onsubmit = |e: SubmitEvent| {
        e.prevent_default();
        let event: SubmitEvent = e.clone();
        let target = &event.target().unwrap();
        log!("submit", target);
        log!("no debugger is pain in the ... is't like the old days! ");
        log!(e);
    };
    let handle_submit = Callback::from(onsubmit);

    let model_ = model.clone();
    let onkeypress = {
        Callback::from(move |event: KeyboardEvent| {
            let target: EventTarget = (&event.target().unwrap()).clone();
            let html_target = target.dyn_into::<HtmlInputElement>().unwrap();
            let name = html_target.clone().name();
            let value = html_target.clone().value();
            let action = LoginAction {
                key: name,
                value: value,
            };
            model_.dispatch(action)
        })
    };

    let model_ = model.clone();
    let onchange = {
        Callback::from(move |event: Event| {
            let target: EventTarget = event.target().unwrap().clone();
            let html_target = target.dyn_into::<HtmlInputElement>().unwrap();
            let name = html_target.clone().name();
            let value = html_target.clone().value();
            let action = LoginAction {
                key: name,
                value: value,
            };
            model_.dispatch(action)
        })
    };

    html!(<div class="min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <img class="mx-auto h-12 w-auto" src="/static/drums_icon.png" alt="Workflow" />
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">{"Sign in to your account"}</h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          {" Or "}
          <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500"> {"register"} </a>
        </p>
      </div>
      <form class="mt-8 space-y-6" action="/api/auth/login"
       method="POST"
       onsubmit={handle_submit}
       onchange={onchange}
       onkeypress={onkeypress}>
        <input type="hidden" name="remember" value="true" />
        <div class="rounded-md shadow-sm -space-y-px">
          <div>
            <label for="email-address" class="sr-only">{"Email address"}</label>
            <input id="email-address" name="email" type="email" autocomplete="email" required={true
            } class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="Email address" />
          </div>
          <div>
            <label for="password" class="sr-only">{"Password"}</label>
            <input id="password" name="password" type="password" autocomplete="current-password" required={true} class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="Password" />
          </div>
        </div>

        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <input id="remember-me" name="remember-me" type="checkbox" class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded" />
            <label for="remember-me" class="ml-2 block text-sm text-gray-900"> {"Remember me"} </label>
          </div>

          <div class="text-sm">
            <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500"> {"Forgot your password?"} </a>
          </div>
        </div>

        <div>
          <button type="submit" class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            <span class="absolute left-0 inset-y-0 flex items-center pl-3">
              <svg class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd" />
              </svg>
            </span>
            {"Sign in"}
          </button>
        </div>
      </form>
    </div>
  </div>)
}
