use crate::components::{create_post, index, navbar};
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
    html! {
        <div class="bg-dark-purple">
            <div class="fixed bg-opacity-25 bg-purple w-full">
           <navbar::Navbar/>
           </div>
           <div class="text-white pt-12 pl-12 pr-12">
            <BrowserRouter>
          <div class="flex flex-col md:flex-row">
          <div class="w-0 md:w-1/4 p-2 hidden md:block">
          </div>
          <div class="w-full p-2">
                <Switch<MainRoute> render={switch}  />
          </div>
          <div class="w-0 md:w-1/4 p-2 hidden md:block center-content">
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
