use crate::components::{index, navbar};
use types::PostResponse;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/:page")]
    Index { page: u8 },
    #[at("/")]
    Home,
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index { page } => html! {<> <index::Index page={page} /> </>},
        MainRoute::Home => html! { <> <index::Index/> </> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="fixed bg-opacity-25 bg-gray-700 w-full">
           <navbar::Navbar/>
           </div>
           <div class="text-gray-100 pt-12 pl-12 pr-12">
            <BrowserRouter>
                <Switch<MainRoute> render={switch}  />
            </ BrowserRouter>
            </div>
            </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
