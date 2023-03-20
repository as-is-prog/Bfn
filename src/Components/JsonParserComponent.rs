use serde::{Deserialize, Serialize};

use yew::html;
use yew::prelude::*;
use yew::{function_component, use_state, Callback, Html, InputEvent};

use web_sys::HtmlTextAreaElement;

#[derive(Serialize, Deserialize)]
struct MyData {
    field1: String,
    field2: i32,
}

#[function_component(JsonParserForm)]
pub fn json_parse_form() -> Html {
    let json_str = use_state(|| "".to_string());
    let parsed_str = use_state(|| "".to_string());

    let oninput = {
        let json_clone = json_str.clone();
        let parsed_clone = parsed_str.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            let value = input.value();

            let parsed: Result<MyData, serde_json::Error> = serde_json::from_str(&value);

            let set_str = match parsed {
                Ok(data) => format!("{}", data.field1),
                Err(e) => format!("{}", e),
            };
            json_clone.set(value);
            parsed_clone.set(set_str);
        })
    };

    html! {
      <form>
        <label for="title" class="form-label">{"タイトル"}</label>
        <div>
            <textarea value={(*json_str).clone()} oninput={oninput} />
        </div>
        <div>
          {&*parsed_str}
        </div>
        <button type="submit">{"パース"}</button>
      </form>
    }
}
