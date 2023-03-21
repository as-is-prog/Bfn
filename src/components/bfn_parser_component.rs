use gloo_file::{callbacks::FileReader, File};
use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::events::Event;
use yew::prelude::*;
use yew::InputEvent;
use yew::{html, Component, Context, Html};

use web_sys::HtmlTextAreaElement;

use crate::bfn::bfn_parser;

pub enum UIMsg {
    JsonChanged(String),
    FileSelected(Option<File>),
    FileLoaded(Vec<u8>),
}

pub struct BfnParserForm {
    json_str: String,
    parsed_str: String,
    visualize_vec: Vec<(String, String)>,
    file_vec: Vec<u8>,
    read_tasks: Vec<(String, FileReader)>,
}

impl Component for BfnParserForm {
    type Message = UIMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            json_str: DEFAULT_JSON_VALUE.to_string(),
            parsed_str: "".to_string(),
            visualize_vec: Vec::new(),
            file_vec: Vec::new(),
            read_tasks: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        fn update_visualize(form: &mut BfnParserForm) -> bool {
            let json_parsed_value = bfn_parser::parse_json(&form.json_str);

            match json_parsed_value {
                Ok(data) => {
                    let visual_pair =
                        bfn_parser::convert_binary_to_bfn_visualize_pair(&form.file_vec, &data);
                    let set_str = visual_pair
                        .iter()
                        .map(|f| format!("{}: {}", f.0, f.1))
                        .collect::<Vec<_>>()
                        .join("¥n");

                    form.visualize_vec = visual_pair;
                    form.parsed_str = set_str;
                    return true;
                }
                Err(e) => {
                    form.parsed_str = format!("{}", e);
                    return true;
                }
            };
        }

        match msg {
            UIMsg::JsonChanged(value) => {
                self.json_str = value;
                return update_visualize(self);
            }
            UIMsg::FileSelected(file) => {
                match file {
                    Some(f) => {
                        let link = _ctx.link().clone();
                        log::info!("Update: {:?}", f);

                        let read_task = {
                            gloo_file::callbacks::read_as_bytes(&f, move |res| {
                                link.send_message(UIMsg::FileLoaded(res.unwrap()));
                            })
                        };

                        self.read_tasks.push((f.name(), read_task));

                        return true;
                    }
                    None => {
                        return false;
                    }
                };
            }
            UIMsg::FileLoaded(d) => {
                log::info!("loaded: {:?}", d);
                self.read_tasks.remove(0);
                self.file_vec = d;

                return update_visualize(self);
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let my_oninput = _ctx.link().callback(|event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            let value = input.value();

            return UIMsg::JsonChanged(value);
        });

        let my_onchange = _ctx.link().callback(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(files) = input.unwrap().files() {
                let file = files.get(0);
                match file {
                    Some(f) => {
                        let g_file = File::from(f);
                        log::info!("Update: {:?}", g_file);
                        return UIMsg::FileSelected(Some(g_file));
                    }
                    None => {}
                }
            }

            return UIMsg::FileSelected(None);
        });

        html! {
          <form>
            <label class="form-label">{"ファイル選択"}</label>
            <input type="file" onchange={my_onchange} /><br/>
            <br/>
            <label class="form-label">{"Json入力"}</label>
            <div>
                <textarea oninput={my_oninput} value={self.json_str.clone()} style={"height: 300px; width: 380px;"} />
            </div>
            <br/>
            <label class="form-label">{"結果表示"}</label>
            <div>
                <ul>
                {self.visualize_vec.iter().map(|v| html! {
                    <li>{v.0.clone()}{":"}{v.1.clone()}</li>
                }).collect::<Html>()}
                </ul>
            </div>
          </form>
        }
    }
}

const DEFAULT_JSON_VALUE: &str = r#"{
    "version": "0",
    "name": "BitmapFormat",
    "defines": [
        {
            "name": "FileHeader",
            "children": [
                {
                    "BfnJsonByte": {
                        "name": "FileType",
                        "len": 2
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "FileSize",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "Reserved",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "DataOffset",
                        "len": 4
                    }
                }
            ]
        },
        {
            "name": "InfoHeader",
            "children": [
                {
                    "BfnJsonNumber": {
                        "name": "InfoHeaderSize",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "ImageWidth",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "ImageHeight",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "Planes",
                        "len": 2
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "BitCount",
                        "len": 2
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "Compression",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "ImageSize",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "XPixelPerMeter",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "YPixelPerMeter",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "ColorUsed",
                        "len": 4
                    }
                },
                {
                    "BfnJsonNumber": {
                        "name": "ColorImportant",
                        "len": 4
                    }
                }
            ]
        }
    ],
    "children": [
        {
            "BfnJsonInstance": {
                "name": "FileHeader",
                "define_name": "FileHeader"
            }
        },
        {
            "BfnJsonInstance": {
                "name": "InfoHeader",
                "define_name": "InfoHeader"
            }
        },
        {
            "BfnJsonAnchorLenMultipleByte": {
                "name": "ColorPalette",
                "len": "ColorUsed",
                "multiple_num": 4
            }
        },
        {
            "BfnJsonAnchorLenMultipleByte": {
                "name": "ImageData",
                "len": "ImageSize",
                "multiple_num": 1
            }
        }
    ]
}"#;
