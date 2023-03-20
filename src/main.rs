use yew::prelude::*;

pub mod bfn;
mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <components::json_parser_component::JsonParserForm />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
