use crate::components::post;
use chrono::Utc;
use crate::PostResponse;
use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct IndexPageProps {
    #[prop_or(1)]
    pub page: u8,
}

#[function_component(Index)]
pub fn index(props: &IndexPageProps) -> Html {
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
                    match Request::get(&format!(
                        "http://{}:{}/{}",
                        std::env!("API_HOST"),
                        std::env!("API_PORT"),
                        page_copy
                    ))
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
    match &*(*index) {
        Some(value) => value
            .iter()
            .map(|post| html! { <><post::Post content={(*post).clone()} now={Utc::now()}/> </> })
            .collect(),
        None => match (*err).as_ref() {
            Some(e) => {
                html! {
                    <>
                        <h1> {"Failed to load page!"} </h1>
                        <p> {e} </p>
                    </>
                }
            }
            None => {
                html! {
                    <div>
                    <p>{"Couldn't load this page."}</p>
                    </div>
                }
            }
        },
    }
}
