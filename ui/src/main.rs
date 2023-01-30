use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {

    html! {
        <div>
            <p>{"Hello World"}</p>
        </div>
    }
}

fn main() {

    yew::Renderer::<App>::new().render();
}
