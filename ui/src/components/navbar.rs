use yew::{function_component, Properties, html, Html, UseStateHandle, Callback};

#[derive(PartialEq, Properties)]
pub struct NavbarProperties{
    pub is_dark: Box<UseStateHandle<bool>>,
}

#[function_component(Navbar)]
pub fn navbar_component(props: &NavbarProperties) -> Html {
    let NavbarProperties { is_dark } = props;
    let is_dark_clone = is_dark.clone();
    let toggle_theme = Callback::from(move |_| {
        let is_dark = is_dark_clone.clone();
       is_dark.set(!*(*is_dark_clone)); 
    });
    html! {
            <nav class="bg-grey-700 dark:bg-dark-grey shadow-lg content-center z-20 rounded-lg">
                <div class="container mx-auto w-full inline-block text-center">
      <a href="/" class="text-white text-xl font-bold p-4 m-2">{"kouchan"}</a>

      <div class="text-white sm:self-center p-2 text-l sm:border-none">
            <div >
                <button type="button" onclick={toggle_theme}> {*(*is_dark.clone())  } </button>
            </div>
            <div >
                <a href="/post">
                <button class="dark:bg-gray-500 dark:hover:bg-gray-800 w-3/4 h-12 text-white font-bold py-2 px-4 self-center rounded-full">
                <span class="text-xl">{"Post"}</span>
</button>
                </a>
            </div>
      </div>
      </div>
    </nav>
        }
}
