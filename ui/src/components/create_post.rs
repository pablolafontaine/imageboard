use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlInputElement, SubmitEvent};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html};
use yew_router::prelude::Redirect;

use crate::MainRoute;

#[function_component(CreatePost)]
pub fn create_post_component() -> Html {
    let title_ref = use_node_ref();
    let text_ref = use_node_ref();
    let file_ref = use_node_ref();
    let result = Box::new(use_state(|| None));
    let err = Box::new(use_state(|| None));
    let title_clone = title_ref.clone();
    let text_clone = text_ref.clone();
    let file_clone = file_ref.clone();
    let result_clone = result.clone();
    let err_clone = err.clone();

    let submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let title_clone = title_clone.clone();
        let text_clone = text_clone.clone();
        let result = result_clone.clone();
        let file_clone = file_clone.clone();
        let err = err_clone.clone();

        spawn_local(async move {
            'stop: {
                let Some(tr) = title_clone.cast::<HtmlInputElement>() else {
                err.set(Some(String::from("Failed to cast to HtmlInputElement")));
                break 'stop;
            };
                let title_ref = tr.value();
                let Some(txtr) = text_clone.cast::<HtmlInputElement>() else {
                err.set(Some(String::from("Failed to cast to HtmlInputElement")));
                break 'stop;
            };
                let text_ref = txtr.value();
                let Some(fr) = file_clone.cast::<HtmlInputElement>() else {
                err.set(Some(String::from("Failed to cast to HtmlInputElement")));
                        break 'stop;
                    };
                let Some(file_list) = fr.files() else {
                err.set(Some(String::from("Failed to get FileList from file input!")));
                        break 'stop;
                      };
                let Some(file) = file_list.get(0) else {
                err.set(Some(String::from("Failed to get file from FileList")));
                        break 'stop;
                      };
                let file_ref = file;
                let Ok(data) = FormData::new() else {
                err.set(Some(String::from("Failed to create new FormData!")));
                            break 'stop;
                      };
                if let Err(e) = data.append_with_str("title", &title_ref) {
                    err.set(Some(e.as_string().unwrap_or_else(|| {
                        String::from("Error appending title to FormData!")
                    })));
                    break 'stop;
                }
                if let Err(e) = data.append_with_str("text", &text_ref) {
                    err.set(Some(e.as_string().unwrap_or_else(|| {
                        String::from("Error appending text to FormData!")
                    })));
                    break 'stop;
                }
                if let Err(e) = data.append_with_blob("file", &file_ref) {
                    err.set(Some(e.as_string().unwrap_or_else(|| {
                        String::from("Error appending file to FormData!")
                    })));
                    break 'stop;
                }
                let response = reqwasm::http::Request::post(&format!(
                    "http://{}:{}/upload",
                    std::env!("API_HOST"),
                    std::env!("API_PORT")
                ))
                .body(data)
                .send()
                .await;
                let Ok(res) = response else {
                                        err.set(Some(String::from("Failed request to API!")));
                                        break 'stop;
                                    };
                let Ok(txt) = res.text().await else {
                                        err.set(Some(String::from("Failed to retrieve text from response!")));
                                        break 'stop;
                                        };
                result.set(Some(txt));
            }
        })
    });

    //let path = ;
    match &*(*result) {
        Some(_) => {
            html! {
            <Redirect<MainRoute> to={MainRoute::Home}/>
            }
        }
        None => match &*(*err) {
            Some(e) => {
                html! {
                <>
                    <p>
                    {e}
                    </p>
                <form onsubmit={submit}>
                     <input type="text" placeholder="Title (Max: 100 Characters)" required=true maxlength="100" ref={title_ref}/>
                     <input type="text" placeholder="Description (Max: 2000 Characters)" required=true maxlength="2000" ref={text_ref} />
                     <input type="file" accept="image/png, image/jpeg" required=true ref={file_ref}/>
                     <input type="submit" value="Upload" />
                 </form>
                 </>
                }
            }
            None => {
                html! {
                <>
                <form onsubmit={submit}>
                     <input type="text" placeholder="Title (Max: 100 Characters)" required=true maxlength="100" ref={title_ref}/>
                     <input type="text" placeholder="Description (Max: 2000 Characters)" required=true maxlength="2000" ref={text_ref} />
                     <input type="file" accept="image/png, image/jpeg" required=true ref={file_ref}/>
                     <input type="submit" value="Upload" />
                 </form>
                 </>
                }
            }
        },
    }
}
