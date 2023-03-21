use yew::prelude::*;

pub mod bfn;
mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Bfn - Binary format notation" }</h1>
            <components::bfn_parser_component::BfnParserForm />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // ロガーの初期化

    yew::Renderer::<App>::new().render();
}
