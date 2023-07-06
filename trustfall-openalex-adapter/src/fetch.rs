use reqwest::Client;
use serde_json::Value;
use lazy_static::lazy_static;
use crate::vertex::{
    Vertex, Work, Author, Concept, Source, Institution, Publisher, Funder
};

lazy_static! {
    static ref OpenAlexClient: Client = reqwest::Client::new();
}

pub fn(url: &str) -> Vertex {

}