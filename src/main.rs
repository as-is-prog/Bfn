use yew::prelude::*;

mod Components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <Components::JsonParserComponent::JsonParserForm />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
