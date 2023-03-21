use core::panic;
use gloo_file::{callbacks::FileReader, File};
use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::events::Event;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use yew::{Callback, InputEvent};

use web_sys::HtmlTextAreaElement;

use crate::bfn::bfn_parser;

pub enum UIMsg {
    JsonChanged(String),
}

pub struct BfnParserForm {
    json_str: String,
    parsed_str: String,
    read_tasks: Vec<(String, ())>,
}

impl Component for BfnParserForm {
    type Message = UIMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            json_str: "".to_string(),
            parsed_str: "".to_string(),
            read_tasks: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UIMsg::JsonChanged(value) => {
                let parsed = bfn_parser::parse_json(&value);

                let set_str = match parsed {
                    Ok(data) => format!("{:?}", data),
                    Err(e) => format!("{}", e),
                };
                self.json_str = value;
                self.parsed_str = set_str;

                return true; // Re-render
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let my_oninput = _ctx.link().callback(|event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            let value = input.value();

            return UIMsg::JsonChanged(value);
        });

        let my_onchange = {
            Callback::from(move |e: Event| {
                // When events are created the target is undefined, it's only
                // when dispatched does the target get added.
                let target: Option<EventTarget> = e.target();
                // Events can bubble so this listener might catch events from child
                // elements which are not of type HtmlInputElement
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(files) = input.unwrap().files() {
                    let file = files.get(0);
                    match file {
                        Some(f) => {
                            let g_file = File::from(f);
                            log::info!("Update: {:?}", g_file);
                            gloo_file::callbacks::read_as_bytes(&g_file, move |res| {
                                log::info!("callback: {:?}", res);
                            });
                        }
                        None => {}
                    }
                }
            })
        };

        html! {
          <form>
            <label for="title" class="form-label">{"タイトル"}</label>
            <input type="file" onchange={my_onchange} />
            <div>
                <textarea oninput={my_oninput} value={self.json_str.clone()} />
            </div>
            <div>
              {self.parsed_str.clone()}
            </div>
          </form>
        }
    }
}
