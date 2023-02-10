use crate::components::{create_post, index, navbar};
use stdweb::web::{document, INonElementParentNode, IElement};
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
    #[at("/post")]
    Post,
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index { page } => html! {<> <index::Index page={page} /> </>},
        MainRoute::Home => html! { <> <index::Index/> </> },
        MainRoute::Post => html! { <> <create_post::CreatePost /> </> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let is_dark = Box::new(use_state(|| true));
    let is_dark_clone = is_dark.clone();
    let theme = if *(*is_dark) {"dark"} else {"light"};
    if let Some(root) = document().get_element_by_id("root")
    { root.set_attribute("class", theme).unwrap_or_default();}
    html! {
        <div>
           <div class="text-custom-white pt-12 pl-12 pr-12 bg-custom-white dark:bg-custom-black">
            <BrowserRouter>
          <div class="flex flex-col md:flex-row">
          <div class="w-0 md:w-1/4 p-2 hidden md:block">
            <div class="bg-opacity-25 bg-custom-gray-800 w-full">
           <navbar::Navbar is_dark={is_dark_clone} />
           </div>
          </div>
          <div class="w-full p-2">
                <Switch<MainRoute> render={switch}  />
          </div>
          <div class="w-0 lg:w-1/4 p-2 hidden lg:block center-content">
          <div>
          {"Boards"}
          </div>
          </div>
        </div>

            </ BrowserRouter>
            </div>
            </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
