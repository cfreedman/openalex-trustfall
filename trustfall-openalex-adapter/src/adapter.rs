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
use vertex::{
  Vertex, Work, Author, Concept, Source, Institution, Publisher, Funder
};

lazy_static! {
  static ref OA_Client: Client = reqwest::Client::new()
}


fn get_item_property(item_vertex: &Vertex, field_name: &str) -> FieldValue {
  
  
}

fn get_funder_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  match field_name {
    "country_code" => vertex[country_code].into(),
    _ => unreachable!("Funder property {field_name}")
  }
}

fn property_mapper<'a>(
  ctx: DataContext<Vertex<'a>>,
  field_name: &str,
  property_getter: fn(&Vertex<'a>, &str) -> FieldValue,
) -> (DataContext<Vertex<'a>>, FieldValue) {
  let value = match ctx.active_vertex() {
      Some(vertex) => property_getter(vertex, field_name),
      None => FieldValue::Null,
  };
  (ctx, value)
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
    if property_name.as_ref() == "__typename" {
      Box::new(contexts.map(|ctx: DataContext<Vertex>| match ctx.active_vertex() {
        Some(vertex) => {
          let value = vertex.typename().into();
          (ctx, value)
        }
        None => (ctx, FieldValue::Null)
      }))
    } else {
      let property_name = property_name.clone();
      match type_name.as_ref() {
        "Work" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_work_property)
        })),
        "Author" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_author_property)
        })),
        "Source" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_source_property)
        })),
        "Concept" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_concept_property)
        })),
        "Institution" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_institution_property)
        })),
        "Publisher" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_publisher_property)
        })),
        "Funder" => Box::new(contexts.map(move |ctx: DataContext<Vertex>| {
          property_mapper(ctx, property_name.as_ref(), get_funder_property)
        })),
        _ => unreachable!("resolve_property {type_name} {property_name}"),
      }
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
    match type_name.as_ref() {
      "Work" => match edge_name.as_ref() {
        "Authors" => {}
        "Cited_by" => {}
        "Concepts" => {}
        "Funders" => {}
        "Locations" => {}
        "References" => {}
        "Related" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")

      }
      "Author" => match edge_name.as_ref() {
        "Institution" => {}
        "Works" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Source" => match edge_name.as_ref() {
        "Host" => {}
        "Lineage" => {}
        "Works" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Concept" => match edge_name.as_ref() {
        "Ancestors" => {}
        "Related" => {}
        "Works" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Institution" => match edge_name.as_ref() {
        "Associated" => {}
        "Repositories" => {}
        "Roles" => {}
        "Works" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Publisher" => match edge_name.as_ref() {
        "Lineage" => {}
        "Roles" => {}
        "Sources" => {}
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Funder" => Box::new(contexts.map(|ctx: DataContext<Vertex>| {
        let vertex = ctx.active_vertex();
        let neighbors = match vertex {
          None => Box::new(std::iter::empty()),
          Some(vertex) => {
            let funder = vertex.as_funder.expect("vertex was not a funder");
            let roles = funder.roles.clone();

            let neighbors_iter = roles.into_iter().filter_map(move |role| {
              match OA_Client.get(role.id) {
                Ok(None) => None,
                Ok(Some()) => {}
              }
            });
            Box::new(neighbors_iter)
          }
        };

        (ctx, neighbors)
      }))
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
    match type_name.as_ref() {
      "Institution" => 
      "Publisher" =>
      "Funder" =>
    }
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