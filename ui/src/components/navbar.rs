use yew::{function_component, html, Html};

/*
#[derive(PartialEq, Properties)]
pub struct NavbarProperties{
    pub content: PostResponse,
}
*/

#[function_component(Navbar)]
pub fn navbar_component() -> Html {
    //let PostProperties { content } = props;
    html! {
            <nav class="bg-grey-700 shadow-lg">
                <div class="container mx-auto">
                <div class="sm:flex justify-around">
      <a href="/" class="text-white text-xl font-bold p-2">{"kouchan"}</a>

      <ul class="text-white sm:self-center text-l border-t sm:border-none">
            <li class="sm:inline-block">
                <a href="/post" class="p-2 hover:text-white">{"new post"}</a>
            </li>
      </ul>
      </div>
      </div>
    </nav>
        }
}
