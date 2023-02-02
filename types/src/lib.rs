use serde::{Deserialize, Serialize};
use std::{fmt, error::Error};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PostResponse {
    id: String,
    title: String,
    text: String,
    date: i64,
    img_path: String,
    comments: Vec<String>,
}

impl PostResponse {
    pub async fn new(id: String, title: String, text: String, date: i64, img_path: String, comments: Vec<String>) -> Self{
        PostResponse{
            id,
            title,
            text,
            date,
            img_path,
            comments, 
        }
    }

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CommentResponse{
    poster: String,
    text: String,
    date: i64,
    img_path: String,
    replies: Vec<String>,
}

impl CommentResponse{
    pub async fn new(poster: String, text: String, date: i64, img_path: String, replies: Vec<String>) -> Self{
        CommentResponse{
            poster,
            text,
            date,
            img_path,
            replies,
        }
    }

}
#[derive(Debug)]
pub struct SplitFileExtError;

impl Error for SplitFileExtError{}
impl fmt::Display for SplitFileExtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to split file extension!")
    }
}

#[derive(Debug)]
pub struct FileSizeError;

impl Error for FileSizeError{}
impl fmt::Display for FileSizeError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error uploading image! File size exceeds limit!")
    }
}

#[derive(Debug)]
pub struct ContentLengthError;

impl Error for ContentLengthError{}
impl fmt::Display for ContentLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error uploading post! Content exceeds character length bounds.")
    }
}

#[derive(Debug)]
pub struct GenerationError;

impl Error for GenerationError{}
impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to generate image ID!")
    }
}

#[derive(Debug)]
pub struct PostLoadError;
impl Error for PostLoadError{}
impl fmt::Display for PostLoadError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading post!")
    }
}
