use reqwasm::http::Request;
use types::PostResponse;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
struct PostProperties {
    pub content: PostResponse,
}

#[derive(PartialEq, Properties)]
struct IndexPageProps {
    pub page: u8,
}

#[function_component(Post)]
fn post_component(props: &PostProperties) -> Html {
    let PostProperties { content } = props;
    html! {
        <div class="post">
            <div class="name">{content.title.to_owned()}</div>
            <div class="text">{content.text.to_owned()}</div>
            <div class="date">{content.date.to_owned()}</div>
            <img src={format!("http://localhost:8080/{}", content.img_path.to_owned())}/>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/:page")]
    Index { page: u8 },
    #[at("/")]
    Home,
}

#[function_component(Index)]
fn index(props: &IndexPageProps) -> Html {
    let IndexPageProps { page } = props;
    let index = Box::new(use_state(|| None));
    let err = Box::new(use_state(|| None));
    {
        let page_copy = *page;
        let index = index.clone();
        let err = err.clone();
        use_effect_with_deps(
            move |_| {
                let index = index.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get(&format!("http://localhost:8080/{}", page_copy))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            let post_response: Result<Vec<PostResponse>, _> = response.json().await;
                            match post_response {
                                Ok(value) => {
                                    index.set(Some(value));
                                }
                                Err(e) => err.set(Some(e.to_string())),
                            }
                        }
                        Err(e) => err.set(Some(e.to_string())),
                    }
                });
                || ()
            },
            (),
        )
    };
    match &*(*index){
            Some(value) => value.iter().map(|post| html! { <><Post content={(*post).clone()}/> <p> {"----------------"} </p></> }).collect(),
            None => match (*err).as_ref(){
                Some(e) => {
    html! {
        <>
            <p> {e} </p>
        </>
    }

                },
                None => {
    html! {
        <div>
        <p>{"Couldn't load this page."}</p>
        </div>
    }

                },
            }
        }
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index { page } => html! {<> <Index page={page} /> </>},
        MainRoute::Home => html! { <Redirect<MainRoute> to={MainRoute::Index { page: 1 } }/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
