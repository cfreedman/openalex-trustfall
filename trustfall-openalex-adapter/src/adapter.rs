use std::{fs, rc::Rc, sync::Arc};

use reqwest::Client;
use serde_json::Value;
use trustfall_core::{
  interpreter::{
      Adapter, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo, ResolveInfo,
      VertexIterator,
  },
  ir::{EdgeParameters, FieldValue},
};

lazy_static! {
  static ref OA_Client: Client = reqwest::Client::new()
}

pub enum Vertex {
  OpenAlexWork(Rc<Value>),
  OpenAlexAuthor(Rc<Value>),
  OpenAlexConcept(Rc<Value>),
  OpenAlexSource(Rc<Value>),
  OpenAlexInstitution(Rc<Value>),
  OpenAlexPublisher(Rc<Value>),
  OpenAlexFunder(Rc<Value>)
}

fn get_item_property(item_vertex: &Vertex, field_name: &str) -> FieldValue {
  
  
}

pub struct AlexAdapter;

impl AlexAdapter {
  
  pub fn new() -> Self {
    Self
  }

  fn get_entity(&self, id: &str) -> VertexIterator<'static, Vertex> {
    let entity = OA_Client
      .get("https://api.openalex.org/")
      .send()
      .json();
    entity
  }
}

impl Adapter<'static> for AlexAdapter {
  type Vertex = Vertex;

  fn resolve_starting_vertices(
    &self,
    edge_name: &Arc<str>,
    parameters: &EdgeParameters,
    _resolve_info: &ResolveInfo,
  ) -> VertexIterator<'static, Self::Vertex> {
    match edge_name.as_ref() {
      "OpenAlexIDSearch" => {
        let id = parameters["id"].as_str().unwrap();
        self.idsearch(id)
      }
      "OpenAlexRandom" => {
        let entity_group = parameters["entity_group"].as_str().unwrap();
        self.random(entity_group)
      }
      "OpenAlexSearch" => {
        let entity_group = parameters["entity_group"].as_str().unwrap();
        let search = parameters["searc"].as_str().unwrap();
        self.search(entity_group, search)
      }
    }
  }

  fn resolve_property(
    &self,
    contexts: ContextIterator<'static, Self::Vertex>,
    type_name: &Arc<str>,
    property_name: &Arc<str>,
    _resolve_info: &ResolveInfo,
  ) -> ContextOutcomeIterator<'static, Self::Vertex, FieldValue> {
    match (type_name.as_ref(), property_name.as_ref()) {
      (_, "___typename") => Box::new(contexts.map()),
      // Shared properties
      (
        "OpenAlexWork" | "OpenAlexAuthor" | "OpenAlexSource" | "OpenAlexInstitution" | "OpenAlexConcept" | "OpenAlexPublisher" | "OpenAlexFunder",
        "id" | "ids" | "display_name" | "cited_by_count" | "counts_by_year" | "created_date" | "updated_date",
      ) => impl_item_property!(contexts, id),
      // Works properties
      (
        "OpenAlexWork",
        "abstract" | "apc_payment" | "biblio" | "open_acess" | "publication_date" | "publication_year" | "title" | "type" | "is_oa" | "license" | "url" | "version"
      ) => imply_item_property!(context, id),
      // Author properties
      (
        "OpenAlexAuthor",
        "display_name_alternatives" | "summary_stats" | "works_count"
      ) => impl_item_property!(contexts, id),
      // Source properties
      (
        "OpenAlexSource",
        "apc_prices" | "homepage_url" | "is_oa" | "issn" | "issn_l" | "societies" | "summary_stats" | "type" | "works_count"
      ) => impl_item_property!(contexts, id),
      // Institution properties
      (
        "OpenAlexInstitution",
        "country_code" | "display_name_alternatives" | "geo" | "homepage_url" | "ror" | "summary_stats" | "type" | "works_count"
      ) => impl_item_property!(contexts, id),
      // Concept properties
      (
        "OpenAlexConcept",
        "description" | "level" | "summary_stats" | "wikidata" | "works_count"
      ) => impl_item_property!(contexts, id),
      // Publisher properties
      (
        "OpenAlexPublisher",
        "alternative_titles" | "country_codes" | "hierarchy_level" | "summary_stats" | "works_count"
      ) => impl_item_property!(contexts, id),
      // Funder properties
      (
        "OpenAlexFunder",
        "alternative_titles" | "country_code" | "description" | "grants_count" | "homepage_url" | "summary_stats" | "works_count"
      ) => impl_item_property!(contexts, id)
    }
  }

  fn resolve_neighbors(
    &self,
    contexts: ContextIterator<'static, Self::Vertex>,
    type_name: &Arc<str>,
    edge_name: &Arc<str>,
    _parameters: &EdgeParameters,
    _resolve_info: &ResolveEdgeInfo,
  ) -> ContextOutcomeIterator<'static, Self::Vertex, VertexIterator<'static, Self::Vertex>> {
    match (type_name.as_ref(), edge_name.as_ref()) {
      // Works edges
      ("OpenAlexWork", )
    }
  }

  fn resolve_coercion(
    &self,
    contexts: ContextIterator<'static, Self::Vertex>,
    type_name: &Arc<str>,
    coerce_to_type: &Arc<str>,
    _resolve_info: &ResolveInfo,
  ) -> ContextOutcomeIterator<'static, Self::Vertex, bool> {
    let type_name = type_name.clone();
    let coerce_to_type = coerce_to_type.clone();
    let iterator = contexts.map(move |ctx| {
        let vertex = match ctx.active_vertex() {
            Some(t) => t,
            None => return (ctx, false),
        };

        let can_coerce = match (type_name.as_ref(), coerce_to_type.as_ref()) {};

        (ctx, can_coerce)
      }); 

      Box::new(iterator)   
  }


}