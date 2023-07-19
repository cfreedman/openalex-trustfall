use reqwest::{Response, Client};
use serde_json::Value;
use lazy_static::lazy_static;
use crate::vertex::{
    Vertex, Work, Author, Concept, Source, Institution, Publisher, Funder
};

lazy_static! {
    static ref OPEN_ALEX_CLIENT: Client = reqwest::Client::new();
}

pub async fn fetch_vertex<'a>(url: &str, deserialize_type: &str) -> Vertex<'a> {

    let json_response: reqwest::Response = OPEN_ALEX_CLIENT.get(url)
        .send()
        .await?;

    match deserialize_type {
        "Work" => json_response.json::<Work>().await?,
        "Author" => json_response.json::<Author>().await?,
        "Source" => json_response.json::<Source>().await?,
        "Concept" => json_response.json::<Concept>().await?,
        "Institution" => json_response.json::<Institution>().await?,
        "Publisher" => json_response.json::<Publisher>().await?,
        "Funder" => json_response.json::<Funder>().await?,
        _ => panic!("Invalid deserailize type")
    }
}

pub fn fetch_vertices(url: &str, deserialize_type: &str) -> Vec<Vertex> {

    let json_response = OpenAlexClient.get(url)
        .send()
        .await?
        .json<serde_json::Value>()
        .await?;

    match deserialize_type {
        "Work" => {json_response["results"].map(move |item| {
            let work_item: Vertex::Work = serde_json::from_str(item)?;
            work_item
        })},
        "Author" => {json_response["results"].map(move |item| {
            let author_item: Vertex::Author = serde_json::from_str(item)?;
            author_item
        })},
        "Source" => {json_response["results"].map(move |item| {
            let source_item: Vertex::Source = serde_json::from_str(item)?;
            source_item
        })},
        "Concept" => {json_response["results"].map(move |item| {
            let concept_item: Vertex::Concept = serde_json::from_str(item)?;
            concept_item
        })},
        "Institution" => {json_response["results"].map(move |item| {
            let institution_item: Vertex::Institution = serde_json::from_str(item)?;
            institution_item
        })},
        "Publisher" => {json_response["results"].map(move |item| {
            let publisher_item: Vertex::Publisher = serde_json::from_str(item)?;
            publisher_item
        })},
        "Funder" => {json_response["results"].map(move |item| {
            let funder_item: Vertex::Funder = serde_json::from_str(item)?;
            funder_item
        })},
        _ => "Not listed vertex type"
    }
}