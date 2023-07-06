use std::{fs, rc::Rc, sync::Arc};

use regex::internal::Inst;
use reqwest::Client;
use serde_json::Value;
use lazy_static::lazy_static;
use trustfall_core::{
  interpreter::{
      Adapter, ContextIterator, ContextOutcomeIterator, DataContext, ResolveEdgeInfo, ResolveInfo,
      VertexIterator,
  },
  ir::{EdgeParameters, FieldValue},
};
use crate::vertex::{
  Vertex, Work, Author, Concept, Source, Institution, Publisher, Funder
};

lazy_static! {
  static ref OA_Client: Client = reqwest::Client::new();
}

fn get_work_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let work = vertex.as_work().expect("Vertex was not a work");
  match field_name {
    "cited_by_count" => work.object.clone().into(),
    "abstract_text" => work.abstract_text.clone().into(),
    "authorships" => work.authorships.clone().into(),
    "apc_payment" => work.apc_payment.clone().into(),
    "best_oa_location" => work.best_oa_location.clone().into(),
    "biblio" => work.biblio.clone().into(),
    "cited_by_api_url" => work.cited_by_api_url.clone().into(),
    "concepts" => work.concepts.clone().into(),
    "corresponding_author_ids" => work.corresponding_author_ids.clone().into(),
    "corresponding_institution_ids" => work.corresponding_institution_ids.clone().into(),
    "doi" => work.doi.clone().into(), 
    "grants" => work.grants.clone().into(),
    "is_paratext" => work.is_paratext.into(), 
    "is_retracted" => work.is_retracted.into(),
    "language" => work.language.clone().into(),
    "locations" => work.locations.clone().into(),
    "mesh" => work.mesh.clone().into(),
    "ngrams_url" => work.ngrams_url.clone().into(), 
    "open_access" => work.open_access.clone().into(), 
    "primary_location" => work.primary_location.clone().into(),
    "publication_date" => work.publication_date.clone().into(), 
    "publication_year" => work.publication_year.into(),
    "referenced_works" => work.referenced_works.clone().into(),
    "related_works" => work.related_works.clone().into(),
    "title" => work.title.clone().into(), 
    "ttype" => work.ttype.clone().into(),
    "is_oa" => work.is_oa.into(), 
    "license" => work.license.clone().into(), 
    "url" => work.url.clone().into(),
    "version" => work.version.clone().into(),
    _ => unreachable!("Work property {field_name}")
  }
}

fn get_author_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let author = vertex.as_author().expect("Vertex was not an author");
  match field_name {
    "object" => author.object.clone().into(),
    "display_name_alternatives" => author.display_name_alternatives.clone().into(),
    "last_known_institution" => author.last_known_institution.clone().into(),
    "orcid" => author.orcid.clone().into(),
    "summary_stats" => author.summary_stats.clone().into(),
    "works_api_url" => author.works_api_url.clone().into(),
    "works_count" => author.works_count.into(),
    _ => unreachable!("Author property {field_name}")
  }
}

fn get_source_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let source = vertex.as_source().expect("Vertex was not a source");
  match field_name {
    "object" => source.object.clone().into(),
    "abreviated_title" => source.abreviated_title.clone().into(),
    "alternative_titles" => source.alternative_titles.clone().into(),
    "apc_payment" => source.apc_payment.clone().into(),
    "apc_usd" => source.apc_usd.into(),
    "country_code" => source.country_code.clone().into(),
    "homepage_url" => source.homepage_url.clone().into(),
    "host_organization" => source.host_organization.clone().into(),
    "host_organization_lineage" => source.host_organization_lineage.clone().into(),
    "host_organization_name" => source.host_organization_name.clone().into(),
    "is_in_doaj" => source.is_in_doaj.into(),
    "is_oa" => source.is_oa.into(),
    "issn" => source.issn.clone().into(),
    "issn_l" => source.issn_l.clone().into(),
    "societies" => source.societies.clone().into(),
    "summary_stats" => source.summary_stats.clone().into(),
    "ttype" => source.ttype.clone().into(),
    "works_api_url" => source.works_api_url.clone().into(),
    "works_count" => source.works_count.into(),
    _ => unreachable!("Source property {field_name}")
  }
}

fn get_concept_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let concept = vertex.as_concept().expect("Vertex was not a concept");
  match field_name {
    "ancestors" => concept.ancestors.clone().into(),
    "object" => concept.object.clone().into(),
    "description" => concept.description.clone().into(),
    "image_thumbnail_url" => concept.image_thumbnail_url.clone().into(),
    "image_url" => concept.image_url.clone().into(),
    "level" => concept.level.into(),
    "related_concepts" => concept.related_concepts.clone().into(),
    "summary_stats" => concept.summary_stats.clone().into(),
    "wikidata" => concept.wikidata.clone().into(),
    "works_api_url" => concept.works_api_url.clone().into(),
    "works_count" => concept.works_count.into(),
    _ => unreachable!("Concept property {field_name}")
  }
}

fn get_institution_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let institution = vertex.as_institution().expect("Vertex was not an institution");
  match field_name {
    "object" => institution.object.clone().into(),
    "associated_institutions" => institution.associated_institutions.clone().into(),
    "country_codes" => institution.country_code.clone().into(),
    "display_name_alternatives" => institution.display_name_alternatives.into(),
    "geo" => institution.geo.clone().into(),
    "homepage_url" => institution.homepage_url.clone().into(),
    "repositories" => institution.repositories.clone().into(),
    "ror" => institution.ror.clone().into(),
    "roles" => institution.roles.clone().into(),
    "ttype" => institution.ttype.clone().into(),
    "summary_stats" => institution.summary_stats.clone().into(),
    "works_api_url" => institution.works_api_url.clone().into(),
    "works_count" => institution.works_count.into(),
    _ => unreachable!("Institution property {field_name}")
  }
}

fn get_publisher_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let publisher = vertex.as_publisher().expect("Vertex was not a publisher");
  match field_name {
    "object" => publisher.object.clone().into(),
    "alternative_titles" => publisher.alternative_titles.clone().into(),
    "country_codes" => publisher.country_codes.clone().into(),
    "hierarchy_level" => publisher.hierarchy_level.into(),
    "image_thumbnail_url" => publisher.image_thumbnail_url.clone().into(),
    "image_url" => publisher.image_url.clone().into(),
    "lineage" => publisher.lineage.clone().into(),
    "parent_publisher" => publisher.parent_publisher.clone().into(),
    "roles" => publisher.roles.clone().into(),
    "sources_api_url" => publisher.sources_api_url.clone().into(),
    "summary_stats" => publisher.summary_stats.clone().into(),
    "works_count" => publisher.works_count.into(),
    _ => unreachable!("Publisher property {field_name}")
  }
}

fn get_funder_property(vertex: &Vertex, field_name: &str) -> FieldValue {
  let funder = vertex.as_funder().expect("Vertex was not a funder");
  match field_name {
    "object" => funder.object.clone().into(),
    "alternative_titles" => funder.alternative_titles.clone().into(),
    "country_code" => funder.country_code.clone().into(),
    "description" => funder.description.clone().into(),
    "grants_count" => funder.grants_count.into(),
    "homepage_url" => funder.homepage_url.clone().into(),
    "image_thumbnail_url" => funder.image_thumbnail_url.clone().into(),
    "image_url" => funder.image_url.clone().into(),
    "roles" => funder.roles.clone().into(),
    "summary_stats" => funder.summary_stats.clone().into(),
    "works_count" => funder.works_count.into(),
    _ => unreachable!("Funder property {field_name}")
  }
}

fn property_mapper<'a>(
  ctx: DataContext<Vertex<'a>>,
  field_name: &str,
  property_getter: fn(&Vertex, &str) -> FieldValue,
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
  type Vertex = Vertex<'static>;

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
        "Authors" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work")
              let author_ids = work.authorships.iter().map(|authorship| {
                authorship.author.id
              });

              let neighbors_iter =
                author_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Cited_by" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work")
              let cited_by_ids = work.cited_by_api_url

              let neighbors_iter =
                author_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Concepts" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work")
              let concept_ids = work.concepts.iter().map(|concept| {
                concept.id
              });

              let neighbors_iter =
                concept_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Funders" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work")
              let funder_ids = work.grants.iter().map(|grant| {
                grant.funder
              });

              let neighbors_iter =
                funder_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "References" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work");
              let reference_ids = work.referenced_works;

              let neighbors_iter =
                reference_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Related" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let work = vertex.as_work().expect("vertex was not a work")
              let related_ids = work.related_works;

              let neighbors_iter =
                related_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")

      }
      "Author" => match edge_name.as_ref() {
        "Institution" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let author = vertex.as_author().expect("vertex was not a work")
              let institution_id = author.last_known_institution.id;

              let neighbors_iter = match institution_id {
                Ok(None) => None,
                Ok(Some(institution_id)) => {}
                Err(error) => {
                  eprintln!(
                    "API error while fetching author with id {author_id}: {author_id}"
                  );
                  None
                }
              }

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),
        "Works" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let author = vertex.as_author().expect("vertex was not a work")
              let works_ids = author.works_api_url;

              let neighbors_iter =
                works_ids.into_iter().filter_map(move |author_id| {
                  match author_id {
                    Ok(None) => None,
                    Ok(Some(author)) => {None},
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }
      "Source" => match edge_name.as_ref() {
        "Host" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let source = vertex.as_source().expect("vertex was not a work")
              let host_id = source.host_organization;

              let neighbors_iter = match host_id {
                Ok(None) => None,
                Ok(Some(institution)) => {}
                Err(error) => {
                  eprintln!(
                    "API error while fetching author with id {author_id}: {author_id}"
                  );
                  None
                }
              }

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Lineage" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let source = vertex.as_source().expect("vertex was not a work")
              let host_lineage_ids = source.host_organization_lineage;

              let neighbors_iter = 
                host_lineage_ids.into_iter().filter_map(move |host_id| {
                  match host_id {
                    Ok(None) => None,
                    Ok(Some(institution)) => {}
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };
        "Works" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let source = vertex.as_source().expect("vertex was not a work")
              let works_api_url = source.works_api_url;

              let neighbors_iter = match works_api_url {
                Ok(None) => None,
                Ok(Some(institution)) => {}
                Err(error) => {
                  eprintln!(
                    "API error while fetching author with id {author_id}: {author_id}"
                  );
                  None
                }
              }

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),
        _ => unreachable!("resolve_neighbors {type_name} {edge_name}")
      }

      "Concept" => match edge_name.as_ref() {
        "Ancestors" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let concept = vertex.as_concept().expect("vertex was not a concept")
              let ancestors = source.ancestors;

              let neighbors_iter = 
                ancestors.into_iter().filter_map(move |ancestor| {
                  match ancestor {
                    Ok(None) => None,
                    Ok(Some(institution)) => {}
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Related" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let concept = vertex.as_concept().expect("vertex was not a concept")
              let related_concepts = source.related_concepts;

              let neighbors_iter = 
                related_concepts.into_iter().filter_map(move |related| {
                  match ancestor {
                    Ok(None) => None,
                    Ok(Some(institution)) => {}
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),

        "Works" => Box::new(contexts.map(move |ctx| {
          let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
            None => Box::new(std::iter::empty()),
            Some(vertex) => {
              let concept = vertex.as_concept().expect("vertex was not a concept")
              let works_api_url = source.works_api_url;

              let neighbors_iter = 
                works_api_url.into_iter().filter_map(move |ancestor| {
                  match ancestor {
                    Ok(None) => None,
                    Ok(Some(institution)) => {}
                    Err(error) => {
                      eprintln!(
                        "API error while fetching author with id {author_id}: {author_id}"
                      );
                      None
                    }
                  }
                });

              Box::new(neighbors_iter)
            }
          };

          (ctx, neighbors)
        })),
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

    let iterator = contexts.map(move |ctx| {
        let vertex = match ctx.active_vertex() {
            Some(t) => t,
            None => return (ctx, false),
        };

        let can_coerce = match (type_name.as_ref(), coerce_to_type.as_ref()) {
          ("Institution", "Funder") => vertex.as_funder().is_some(),
          ("Institution", "Publisher") => vertex.as_publisher().is_some(),
          ("Funder", "Institution") => vertex.as_institution().is_some(),
          ("Funder", "Publisher") => vertex.as_publisher().is_some(),
          ("Publisher", "Institution") => vertex.as_institution().is_some(),
          ("Publisher", "Funder") => vertex.as_funder().is_some(),
          unhandled => unreachable!("{:?}", unhandled)
        };

        (ctx, can_coerce)
      }); 

      Box::new(iterator)   
  }


}