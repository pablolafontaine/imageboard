use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PostResponse {
    title: String,
    text: String,
    date: i64,
    img_path: String,
}

impl PostResponse {
    pub async fn new(title: String, text: String, date: i64, img_path: String) -> Self{
        PostResponse{
            title,
            text,
            date,
            img_path,
        }
    }

}

