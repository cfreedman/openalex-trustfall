use std::sync::Arc;

use crate::{
    fetch::{fetch_vertex, fetch_vertices},
    vertex::{Vertex, VertexKind},
};

use trustfall_core::{
    interpreter::{
        Adapter, ContextIterator, ContextOutcomeIterator, DataContext, ResolveEdgeInfo,
        ResolveInfo, VertexIterator,
    },
    ir::{EdgeParameters, FieldValue},
};

fn get_work_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let work = vertex.as_work().expect("Vertex was not a work");
    match field_name {
        "cited_by_count" => work.object.clone().cited_by_count.into(),
        "created_date" => work.object.clone().created_date.into(),
        "display_name" => work.object.clone().display_name.into(),
        "id" => work.object.clone().id.into(),
        "ids_doi" => work.object.clone().ids.doi.into(),
        "ids_mag" => work.object.clone().ids.mag.into(),
        "ids_openalex" => work.object.clone().ids.openalex.into(),
        "ids_pmid" => work.object.clone().ids.pmid.into(),
        "ids_pmcid" => work.object.clone().ids.pmcid.into(),
        "updated_date" => work.object.clone().updated_date.into(),
        "abstract_text" => work.abstract_inverted_index.clone().into(),
        "apc_list_value" => work
            .apc_list
            .clone()
            .and_then(|payment| payment.value)
            .into(),
        "apc_list_currency" => work
            .apc_list
            .clone()
            .and_then(|payment| payment.currency)
            .into(),
        "apc_list_provenance" => work
            .apc_list
            .clone()
            .and_then(|payment| payment.provenance)
            .into(),
        "apc_list_value_usd" => work
            .apc_list
            .clone()
            .and_then(|payment| payment.value_usd)
            .into(),
        "apc_payment_value" => work
            .apc_paid
            .clone()
            .and_then(|payment| payment.value)
            .into(),
        "apc_payment_currency" => work
            .apc_paid
            .clone()
            .and_then(|payment| payment.currency)
            .into(),
        "apc_payment_provenance" => work
            .apc_paid
            .clone()
            .and_then(|payment| payment.provenance)
            .into(),
        "apc_payment_value_usd" => work
            .apc_paid
            .clone()
            .and_then(|payment| payment.value_usd)
            .into(),
        "best_oa_location_is_oa" => work
            .best_oa_location
            .clone()
            .and_then(|location| location.is_oa)
            .into(),
        "best_oa_location_landing_page_url" => work
            .best_oa_location
            .clone()
            .and_then(|location| location.landing_page_url)
            .into(),
        "best_oa_location_license" => work
            .best_oa_location
            .clone()
            .and_then(|location| location.license)
            .into(),
        "best_oa_location_pdf_url" => work
            .best_oa_location
            .clone()
            .and_then(|location| location.pdf_url)
            .into(),
        "best_oa_location_version" => work
            .best_oa_location
            .clone()
            .and_then(|location| location.version)
            .into(),
        "biblio_volume" => work.biblio.clone().and_then(|biblio| biblio.volume).into(),
        "biblio_issue" => work.biblio.clone().and_then(|biblio| biblio.issue).into(),
        "biblio_first_page" => work
            .biblio
            .clone()
            .and_then(|biblio| biblio.first_page)
            .into(),
        "biblio_last_page" => work
            .biblio
            .clone()
            .and_then(|biblio| biblio.last_page)
            .into(),
        "doi" => work.doi.clone().into(),
        "is_paratext" => work.is_paratext.into(),
        "is_retracted" => work.is_retracted.into(),
        "language" => work.language.clone().into(),
        "open_access_is_oa" => work
            .open_access
            .clone()
            .and_then(|open_access| open_access.is_oa)
            .into(),
        "open_access_oa_status" => work
            .open_access
            .clone()
            .and_then(|open_access| open_access.oa_status)
            .into(),
        "open_access_oa_url" => work
            .open_access
            .clone()
            .and_then(|open_access| open_access.oa_url)
            .into(),
        "open_access_fulltext" => work
            .open_access
            .clone()
            .and_then(|open_access| open_access.any_repository_has_fulltext)
            .into(),
        "publication_date" => work.publication_date.clone().into(),
        "publication_year" => work.publication_year.into(),
        "referenced_works" => work.referenced_works.clone().into(),
        "related_works" => work.related_works.clone().into(),
        "title" => work.title.clone().into(),
        "ttype" => work.ttype.clone().into(),
        "is_oa" => work.is_oa.into(),
        "license" => work.license.clone().into(),
        _ => unreachable!("Work property {field_name}"),
    }
}

fn get_author_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let author = vertex.as_author().expect("Vertex was not an author");
    match field_name {
        "cited_by_count" => author.object.clone().cited_by_count.into(),
        "created_date" => author.object.clone().created_date.into(),
        "display_name" => author.object.clone().display_name.into(),
        "id" => author.object.clone().id.into(),
        "ids_doi" => author.object.clone().ids.doi.into(),
        "ids_mag" => author.object.clone().ids.mag.into(),
        "ids_openalex" => author.object.clone().ids.openalex.into(),
        "ids_pmid" => author.object.clone().ids.pmid.into(),
        "ids_pmcid" => author.object.clone().ids.pmcid.into(),
        "updated_date" => author.object.clone().updated_date.into(),
        "display_name_alternatives" => author.display_name_alternatives.clone().into(),
        "orcid" => author.orcid.clone().into(),
        "summary_stats_mean_citeness" => match author
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => author
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => author
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "works_count" => author.works_count.into(),
        _ => unreachable!("Author property {field_name}"),
    }
}

fn get_source_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let source = vertex.as_source().expect("Vertex was not a source");
    match field_name {
        "cited_by_count" => source.object.clone().cited_by_count.into(),
        "created_date" => source.object.clone().created_date.into(),
        "display_name" => source.object.clone().display_name.into(),
        "id" => source.object.clone().id.into(),
        "ids_doi" => source.object.clone().ids.doi.unwrap().into(),
        "ids_mag" => source.object.clone().ids.mag.unwrap().into(),
        "ids_openalex" => source.object.clone().ids.openalex.unwrap().into(),
        "ids_pmid" => source.object.clone().ids.pmid.unwrap().into(),
        "ids_pmcid" => source.object.clone().ids.pmcid.unwrap().into(),
        "updated_date" => source.object.clone().updated_date.into(),
        "abreviated_title" => source.abreviated_title.clone().into(),
        "alternative_titles" => source.alternative_titles.clone().into(),
        "apc_payment" => source
            .apc_payment
            .clone()
            .into_iter()
            .map(|price_obj| price_obj.price.to_string() + " - " + &price_obj.currency)
            .collect::<Vec<String>>()
            .into(),
        "apc_usd" => source.apc_usd.into(),
        "country_code" => source.country_code.clone().into(),
        "homepage_url" => source.homepage_url.clone().into(),
        "host_organization_name" => source.host_organization_name.clone().into(),
        "is_in_doaj" => source.is_in_doaj.into(),
        "is_oa" => source.is_oa.into(),
        "issn" => source.issn.clone().into(),
        "issn_l" => source.issn_l.clone().into(),
        "societies" => source
            .societies
            .clone()
            .into_iter()
            .map(|society| society.url + " - " + &society.organization)
            .collect::<Vec<String>>()
            .into(),
        "summary_stats_mean_citeness" => match source
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => source
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => source
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "ttype" => source.ttype.clone().into(),
        "works_api_url" => source.works_api_url.clone().into(),
        "works_count" => source.works_count.into(),
        _ => unreachable!("Source property {field_name}"),
    }
}

fn get_concept_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let concept = vertex.as_concept().expect("Vertex was not a concept");
    match field_name {
        "cited_by_count" => concept.object.clone().cited_by_count.into(),
        "created_date" => concept.object.clone().created_date.into(),
        "display_name" => concept.object.clone().display_name.into(),
        "id" => concept.object.clone().id.into(),
        "ids_doi" => concept.object.clone().ids.doi.unwrap().into(),
        "ids_mag" => concept.object.clone().ids.mag.unwrap().into(),
        "ids_openalex" => concept.object.clone().ids.openalex.unwrap().into(),
        "ids_pmid" => concept.object.clone().ids.pmid.unwrap().into(),
        "ids_pmcid" => concept.object.clone().ids.pmcid.unwrap().into(),
        "updated_date" => concept.object.clone().updated_date.into(),
        "description" => concept.description.clone().into(),
        "image_thumbnail_url" => concept.image_thumbnail_url.clone().into(),
        "image_url" => concept.image_url.clone().into(),
        "level" => concept.level.into(),
        "summary_stats_mean_citeness" => match concept
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => concept
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => concept
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "wikidata" => concept.wikidata.clone().into(),
        "works_count" => concept.works_count.into(),
        _ => unreachable!("Concept property {field_name}"),
    }
}

fn get_institution_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let institution = vertex
        .as_institution()
        .expect("Vertex was not an institution");
    match field_name {
        "cited_by_count" => institution.object.clone().cited_by_count.into(),
        "created_date" => institution.object.clone().created_date.into(),
        "display_name" => institution.object.clone().display_name.into(),
        "id" => institution.object.clone().id.into(),
        "ids_doi" => institution.object.clone().ids.doi.into(),
        "ids_mag" => institution.object.clone().ids.mag.into(),
        "ids_openalex" => institution.object.clone().ids.openalex.into(),
        "ids_pmid" => institution.object.clone().ids.pmid.into(),
        "ids_pmcid" => institution.object.clone().ids.pmcid.into(),
        "updated_date" => institution.object.clone().updated_date.into(),
        "country_codes" => institution.country_code.clone().into(),
        "display_name_alternatives" => institution.display_name_alternatives.clone().into(),
        "geo_city" => institution.geo.clone().and_then(|geo| geo.city).into(),
        "geo_geonames_city_id" => institution
            .geo
            .clone()
            .and_then(|geo| geo.clone().geonames_city_id)
            .into(),
        "geo_region" => institution.geo.clone().and_then(|geo| geo.region).into(),
        "geo_country_code" => institution
            .geo
            .clone()
            .and_then(|geo| geo.country_code)
            .into(),
        "geo_country" => institution.geo.clone().and_then(|geo| geo.country).into(),
        "geo_latitude" => institution.geo.clone().and_then(|geo| geo.latitude).into(),
        "geo_longitude" => institution.geo.clone().and_then(|geo| geo.longitude).into(),
        "homepage_url" => institution.homepage_url.clone().into(),
        "ror" => institution.ror.clone().into(),
        "ttype" => institution.ttype.clone().into(),
        "summary_stats_mean_citeness" => match institution
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => institution
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => institution
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "works_count" => institution.works_count.into(),
        _ => unreachable!("Institution property {field_name}"),
    }
}

fn get_publisher_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let publisher = vertex.as_publisher().expect("Vertex was not a publisher");
    match field_name {
        "cited_by_count" => publisher.object.clone().cited_by_count.into(),
        "created_date" => publisher.object.clone().created_date.into(),
        "display_name" => publisher.object.clone().display_name.into(),
        "id" => publisher.object.clone().id.into(),
        "ids_doi" => publisher.object.clone().ids.doi.unwrap().into(),
        "ids_mag" => publisher.object.clone().ids.mag.unwrap().into(),
        "ids_openalex" => publisher.object.clone().ids.openalex.unwrap().into(),
        "ids_pmid" => publisher.object.clone().ids.pmid.unwrap().into(),
        "ids_pmcid" => publisher.object.clone().ids.pmcid.unwrap().into(),
        "updated_date" => publisher.object.clone().updated_date.into(),
        "alternative_titles" => publisher.alternative_titles.clone().into(),
        "country_codes" => publisher.country_codes.clone().into(),
        "hierarchy_level" => publisher.hierarchy_level.into(),
        "image_thumbnail_url" => publisher.image_thumbnail_url.clone().into(),
        "image_url" => publisher.image_url.clone().into(),
        "summary_stats_mean_citeness" => match publisher
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => publisher
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => publisher
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "works_count" => publisher.works_count.into(),
        _ => unreachable!("Publisher property {field_name}"),
    }
}

fn get_funder_property(vertex: &Vertex, field_name: &str) -> FieldValue {
    let funder = vertex.as_funder().expect("Vertex was not a funder");
    match field_name {
        "cited_by_count" => funder.object.clone().cited_by_count.into(),
        "created_date" => funder.object.clone().created_date.into(),
        "display_name" => funder.object.clone().display_name.into(),
        "id" => funder.object.clone().id.into(),
        "ids_doi" => funder.object.clone().ids.doi.unwrap().into(),
        "ids_mag" => funder.object.clone().ids.mag.unwrap().into(),
        "ids_openalex" => funder.object.clone().ids.openalex.unwrap().into(),
        "ids_pmid" => funder.object.clone().ids.pmid.unwrap().into(),
        "ids_pmcid" => funder.object.clone().ids.pmcid.unwrap().into(),
        "updated_date" => funder.object.clone().updated_date.into(),
        "alternative_titles" => funder.alternative_titles.clone().into(),
        "country_code" => funder.country_code.clone().into(),
        "description" => funder.description.clone().into(),
        "grants_count" => funder.grants_count.into(),
        "homepage_url" => funder.homepage_url.clone().into(),
        "image_thumbnail_url" => funder.image_thumbnail_url.clone().into(),
        "image_url" => funder.image_url.clone().into(),
        "summary_stats_mean_citeness" => match funder
            .summary_stats
            .clone()
            .and_then(|stats| stats.two_year_mean_citedness)
        {
            Some(num) => FieldValue::Float64(num),
            _ => FieldValue::Null,
        },
        "summary_stats_h_index" => funder
            .summary_stats
            .clone()
            .and_then(|stats| stats.h_index)
            .into(),
        "summary_stats_i10_index" => funder
            .summary_stats
            .clone()
            .and_then(|stats| stats.i10_index)
            .into(),
        "works_count" => funder.works_count.into(),
        _ => unreachable!("Funder property {field_name}"),
    }
}

fn property_mapper(
    ctx: DataContext<Vertex>,
    field_name: &str,
    property_getter: fn(&Vertex, &str) -> FieldValue,
) -> (DataContext<Vertex>, FieldValue) {
    let value = match ctx.active_vertex() {
        Some(vertex) => property_getter(vertex, field_name),
        None => FieldValue::Null,
    };
    (ctx, value)
}

pub struct OpenAlexAdapter;

impl OpenAlexAdapter {
    pub fn new() -> Self {
        Self
    }

    fn search_id(&self, url: String, vertex_kind: String) -> VertexIterator<'static, Vertex> {
        match fetch_vertex(
            url.clone(),
            match vertex_kind.as_str() {
                "Work" => VertexKind::Work,
                "Author" => VertexKind::Author,
                "Source" => VertexKind::Source,
                "Concept" => VertexKind::Concept,
                "Institution" => VertexKind::Institution,
                "Publisher" => VertexKind::Publisher,
                "Funder" => VertexKind::Funder,
                _ => unreachable!("Not a valid vertex kind"),
            },
        ) {
            Ok(vertex) => Box::new(std::iter::once(vertex)),
            Err(e) => {
                eprintln!("API error when fetching or deserializing {url}: {e}");
                Box::new(std::iter::empty())
            }
        }
    }

    fn random(&self, vertex_kind: String) -> VertexIterator<'static, Vertex> {
        let random_url = "https://api.openalex.org/".to_owned()
            + match vertex_kind.as_str() {
                "Work" => "works",
                "Author" => "authors",
                "Source" => "sources",
                "Concept" => "concepts",
                "Institution" => "institutions",
                "Publisher" => "publishers",
                "Funder" => "funders",
                _ => unreachable!("Not a valid vertex kind"),
            }
            + "/random";
        match fetch_vertex(
            random_url.clone(),
            match vertex_kind.as_str() {
                "Work" => VertexKind::Work,
                "Author" => VertexKind::Author,
                "Source" => VertexKind::Source,
                "Concept" => VertexKind::Concept,
                "Institution" => VertexKind::Institution,
                "Publisher" => VertexKind::Publisher,
                "Funder" => VertexKind::Funder,
                _ => unreachable!("Not a valid vertex kind"),
            },
        ) {
            Ok(vertex) => Box::new(std::iter::once(vertex)),
            Err(e) => {
                eprintln!("API error when fetching or deserializing {random_url}: {e}");
                Box::new(std::iter::empty())
            }
        }
    }
}

impl Adapter<'static> for OpenAlexAdapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        _resolve_info: &ResolveInfo,
    ) -> VertexIterator<'static, Self::Vertex> {
        match edge_name.as_ref() {
            "OpenAlexIDSearchWork" => self.search_id(
                parameters
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string(),
                "Work".to_string(),
            ),
            "OpenAlexRandomWork" => self.random("Work".to_string()),
            _ => unreachable!("todo"),
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
            Box::new(
                contexts.map(|ctx: DataContext<Vertex>| match ctx.active_vertex() {
                    Some(vertex) => {
                        let value = vertex.typename().into();
                        (ctx, value)
                    }
                    None => (ctx, FieldValue::Null),
                }),
            )
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
                "Authors" => Box::new(contexts.map(|ctx| {
                    let vertex = ctx.active_vertex();
                    let neighbors: VertexIterator<'static, Self::Vertex> = match vertex
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let author_ids = work.authorships.clone();

                            let neighbors_iter = author_ids.into_iter().filter_map(|authorship| {
                                match fetch_vertex(authorship.author.id.clone(), VertexKind::Author) {
                                    Ok(author_vertex) => Some(author_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {}: {e}", authorship.author.id
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let cited_by_ids = &work.cited_by_api_url;

                            match fetch_vertices(cited_by_ids.clone(), VertexKind::Work) {
                                Ok(cited_by_vertices) => Box::new(cited_by_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {cited_by_ids}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),

                "Concepts" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let concept_ids = work.concepts.clone().into_iter().map(|concept| concept.id);

                            let neighbors_iter = concept_ids.filter_map(move |concept_id| {
                                match fetch_vertex(concept_id.clone(), VertexKind::Concept) {
                                    Ok(concept_vertex) => Some(concept_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {concept_id}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let funder_ids = work.grants.clone().into_iter().map(|grant| grant.funder);

                            let neighbors_iter = funder_ids.filter_map(move |funder_id| {
                                match fetch_vertex(funder_id.clone(), VertexKind::Funder) {
                                    Ok(funder_vertex) => Some(funder_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {funder_id}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let reference_ids = work.referenced_works.clone();

                            let neighbors_iter =
                                reference_ids.into_iter().filter_map(move |reference_id| {
                                    match fetch_vertex(reference_id.clone(), VertexKind::Work) {
                                        Ok(work_vertex) => Some(work_vertex),
                                        Err(e) => {
                                            eprintln!(
                                "API error while fetching or deserializing {reference_id}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let work = vertex.as_work().expect("vertex was not a work");
                            let related_ids = work.related_works.clone();

                            let neighbors_iter =
                                related_ids.into_iter().filter_map(move |related_id| {
                                    match fetch_vertex(related_id.clone(), VertexKind::Work) {
                                        Ok(work_vertex) => Some(work_vertex),
                                        Err(e) => {
                                            eprintln!(
                                "API error while fetching or deserializing {related_id}: {e}"
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

                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            "Author" => match edge_name.as_ref() {
                "Institution" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let author = vertex.as_author().expect("vertex was not a work");
                            match &author.last_known_institution {
                                Some(institution) => {
                                    let institution_id = institution.id.clone();

                                    match fetch_vertex(institution_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => {
                                            Box::new(std::iter::once(institution_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error while fetching or deserializing {institution_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                _ => Box::new(std::iter::empty())
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                "Works" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let author = vertex.as_author().expect("vertex was not a work");
                            let works_ids = &author.works_api_url;

                            match fetch_vertices(works_ids.clone(), VertexKind::Work) {
                                Ok(work_vertices) => Box::new(work_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {works_ids}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            "Source" => match edge_name.as_ref() {
                "Host" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let source = vertex.as_source().expect("vertex was not a work");
                            match &source.host_organization {
                                Some(host_id) => {
                                    match fetch_vertex(host_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => {
                                            Box::new(std::iter::once(institution_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error while fetching or deserializing {host_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                _ => Box::new(std::iter::empty())
                            }
                        }
                    };

                    (ctx, neighbors)
                })),

                "Lineage" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let source = vertex.as_source().expect("vertex was not a work");
                            let host_lineage_ids = source.host_organization_lineage.clone();

                            let neighbors_iter =
                                host_lineage_ids.into_iter().filter_map(move |host_id| {
                                    match fetch_vertex(host_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => Some(institution_vertex),
                                        Err(e) => {
                                            eprintln!(
                                                "API error while fetching or deserializing {host_id}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let source = vertex.as_source().expect("vertex was not a work");
                            let works_api_url = &source.works_api_url;

                            match fetch_vertices(works_api_url.clone(), VertexKind::Work) {
                                Ok(work_vertices) => Box::new(work_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {works_api_url}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            "Concept" => match edge_name.as_ref() {
                "Ancestors" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let concept = vertex.as_concept().expect("vertex was not a concept");
                            let ancestor_ids = concept.ancestors.clone().into_iter().map(|ancestor| ancestor.id);

                            let neighbors_iter = ancestor_ids.filter_map(move |ancestor| {
                                match fetch_vertex(ancestor.clone(), VertexKind::Concept) {
                                    Ok(concept_vertex) => Some(concept_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {ancestor}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> =
                        match ctx.active_vertex() {
                            None => Box::new(std::iter::empty()),
                            Some(vertex) => {
                                let concept =
                                    vertex.as_concept().expect("vertex was not a concept");
                                let related_concept_ids =
                                    concept.related_concepts.clone().into_iter().map(|concept| concept.id);

                                let neighbors_iter =
                                    related_concept_ids.filter_map(move |related_id| {
                                        match fetch_vertex(related_id.clone(), VertexKind::Concept) {
                                            Ok(concept_vertex) => Some(concept_vertex),
                                            Err(e) => {
                                                eprintln!(
                                                    "API error while fetching or deserializing {related_id}: {e}"
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
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let concept = vertex.as_concept().expect("vertex was not a concept");
                            let works_api_url = &concept.works_api_url;

                            match fetch_vertices(works_api_url.clone(), VertexKind::Work) {
                                Ok(work_vertices) => Box::new(work_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {works_api_url}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            "Institution" => match edge_name.as_ref() {
                "Associated" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let institution = vertex
                                .as_institution()
                                .expect("vertex was not an institution");
                            let associated_institution_ids = institution
                                .associated_institutions
                                .clone()
                                .into_iter()
                                .map(|institution| institution.id);

                            let neighbors_iter =
                                associated_institution_ids.filter_map(move |institution_id| {
                                    match fetch_vertex(institution_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => Some(institution_vertex),
                                        Err(e) => {
                                            eprintln!(
                                                "API error while fetching or deserializing {institution_id}: {e}"
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
                "Repositories" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let institution = vertex
                                .as_institution()
                                .expect("vertex was not an institution");
                            let repositories_ids =
                                institution.repositories.clone().into_iter().map(|source| source.id);

                            let neighbors_iter = repositories_ids.filter_map(move |source_id| {
                                match fetch_vertex(source_id.clone(), VertexKind::Source) {
                                    Ok(source_vertex) => Some(source_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {source_id}: {e}"
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
                "Publisher" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let institution =
                                vertex.as_institution().expect("vertex was not institution");
                            let publisher_id_option =
                                institution
                                    .roles
                                    .clone()
                                    .into_iter()
                                    .find_map(|role_object| match role_object.role.as_str() {
                                        "publisher" => Some(role_object.id),
                                        _ => {
                                            eprintln!("Institution has no linked publisher");
                                            None
                                        }
                                    });

                            match publisher_id_option {
                                Some(publisher_id) => {
                                    match fetch_vertex(publisher_id.clone(), VertexKind::Publisher) {
                                        Ok(publisher_vertex) => {
                                            Box::new(std::iter::once(publisher_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error when fetching or deserializing {publisher_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                "Funder" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let institution =
                                vertex.as_institution().expect("vertex was not institution");
                            let funder_id_option =
                                institution
                                    .roles
                                    .clone()
                                    .into_iter()
                                    .find_map(|role_object| match role_object.role.as_str() {
                                        "funder" => Some(role_object.id),
                                        _ => {
                                            eprintln!("Institution has no linked funder");
                                            None
                                        }
                                    });

                            match funder_id_option {
                                Some(funder_id) => {
                                    match fetch_vertex(funder_id.clone(), VertexKind::Funder) {
                                        Ok(funder_vertex) => {
                                            Box::new(std::iter::once(funder_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error when fetching or deserializing {funder_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                "Works" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let institution = vertex
                                .as_institution()
                                .expect("vertex was not an institution");
                            let works_api_url = &institution.works_api_url;

                            match fetch_vertices(works_api_url.clone(), VertexKind::Work) {
                                Ok(work_vertices) => Box::new(work_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {works_api_url}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            "Publisher" => match edge_name.as_ref() {
                "Lineage" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let publisher =
                                vertex.as_publisher().expect("vertex was not an publisher");
                            let lineage_ids = publisher.lineage.clone().into_iter();

                            let neighbors_iter = lineage_ids.filter_map(move |publisher_id| {
                                match fetch_vertex(publisher_id.clone(), VertexKind::Publisher) {
                                    Ok(publisher_vertex) => Some(publisher_vertex),
                                    Err(e) => {
                                        eprintln!(
                                            "API error while fetching or deserializing {publisher_id}: {e}"
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
                "Institution" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let publisher =
                                vertex.as_publisher().expect("vertex was not publisher");
                            let institution_id_option =
                                publisher.roles.clone().into_iter().find_map(|role_object| {
                                    match role_object.role.as_str() {
                                        "institution" => Some(role_object.id),
                                        _ => {
                                            eprintln!("Publisher has no linked institution");
                                            None
                                        }
                                    }
                                });

                            match institution_id_option {
                                Some(institution_id) => {
                                    match fetch_vertex(institution_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => {
                                            Box::new(std::iter::once(institution_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error when fetching or deserializing {institution_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                "Funder" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let publisher =
                                vertex.as_publisher().expect("vertex was not publisher");
                            let funder_id_option = publisher.roles.clone().into_iter().find_map(|role_object| {
                                match role_object.role.as_str() {
                                    "funder" => Some(role_object.id),
                                    _ => {
                                        eprintln!("Publisher has no linked funder");
                                        None
                                    }
                                }
                            });

                            match funder_id_option {
                                Some(funder_id) => {
                                    match fetch_vertex(funder_id.clone(), VertexKind::Funder) {
                                        Ok(funder_vertex) => {
                                            Box::new(std::iter::once(funder_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error when fetching or deserializing {funder_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                "Sources" => Box::new(contexts.map(move |ctx| {
                    let neighbors: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex()
                    {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let publisher =
                                vertex.as_publisher().expect("vertex was not an publisher");
                            let sources_api_url = &publisher.sources_api_url;

                            match fetch_vertices(sources_api_url.clone(), VertexKind::Source) {
                                Ok(sources_vertices) => Box::new(sources_vertices.into_iter()),
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching or deserializing {sources_api_url}: {e}"
                                    );
                                    Box::new(std::iter::empty())
                                }
                            }
                        }
                    };

                    (ctx, neighbors)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },
            "Funder" => match edge_name.as_ref() {
                "Institution" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let funder = vertex.as_funder().expect("vertex was not funder");
                            let institution_id_option =
                                funder.roles.clone().into_iter().find_map(|role_object| {
                                    match role_object.role.as_str() {
                                        "institution" => Some(role_object.id),
                                        _ => {
                                            eprintln!("Publisher has no linked funder");
                                            None
                                        }
                                    }
                                });

                            match institution_id_option {
                                Some(institution_id) => {
                                    match fetch_vertex(institution_id.clone(), VertexKind::Institution) {
                                        Ok(institution_vertex) => {
                                            Box::new(std::iter::once(institution_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                        "API error when fetching or deserializing {institution_id}: {e}"
                      );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                "Publisher" => Box::new(contexts.map(move |ctx| {
                    let neighbor: VertexIterator<'static, Self::Vertex> = match ctx.active_vertex() {
                        None => Box::new(std::iter::empty()),
                        Some(vertex) => {
                            let funder = vertex.as_funder().expect("vertex was not funder");
                            let publisher_id_option = funder.roles.clone().into_iter().find_map(|role_object| {
                                match role_object.role.as_str() {
                                    "publisher" => Some(role_object.id),
                                    _ => {
                                        eprintln!("Publisher has no linked funder");
                                        None
                                    }
                                }
                            });

                            match publisher_id_option {
                                Some(publisher_id) => {
                                    match fetch_vertex(publisher_id.clone(), VertexKind::Publisher) {
                                        Ok(publisher_vertex) => {
                                            Box::new(std::iter::once(publisher_vertex))
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "API error when fetching or deserializing {publisher_id}: {e}"
                                            );
                                            Box::new(std::iter::empty())
                                        }
                                    }
                                }
                                None => Box::new(std::iter::empty()),
                            }
                        }
                    };

                    (ctx, neighbor)
                })),
                _ => unreachable!("resolve_neighbors {type_name} {edge_name}"),
            },

            _ => unreachable!("{type_name} doesn't fall under a vertex type with edge connections"),
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
                ("Institution", "Funder") => match vertex.as_institution() {
                    Some(institution) => {
                        let institution_roles: Vec<String> = institution
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        institution_roles.contains(&String::from("funder"))
                    }
                    None => {
                        eprintln!("vertex was not institution");
                        false
                    }
                },
                ("Institution", "Publisher") => match vertex.as_institution() {
                    Some(institution) => {
                        let institution_roles: Vec<String> = institution
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        institution_roles.contains(&String::from("publisher"))
                    }
                    None => {
                        eprintln!("vertex was not institution");
                        false
                    }
                },
                ("Funder", "Institution") => match vertex.as_funder() {
                    Some(funder) => {
                        let funder_roles: Vec<String> = funder
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        funder_roles.contains(&String::from("institution"))
                    }
                    None => {
                        eprintln!("vertex was not funder");
                        false
                    }
                },
                ("Funder", "Publisher") => match vertex.as_funder() {
                    Some(funder) => {
                        let funder_roles: Vec<String> = funder
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        funder_roles.contains(&String::from("publisher"))
                    }
                    None => {
                        eprintln!("vertex was not funder");
                        false
                    }
                },
                ("Publisher", "Institution") => match vertex.as_publisher() {
                    Some(publisher) => {
                        let publisher_roles: Vec<String> = publisher
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        publisher_roles.contains(&String::from("institution"))
                    }
                    None => {
                        eprintln!("vertex was not publisher");
                        false
                    }
                },
                ("Publisher", "Funder") => match vertex.as_publisher() {
                    Some(publisher) => {
                        let publisher_roles: Vec<String> = publisher
                            .roles
                            .clone()
                            .into_iter()
                            .map(|role_object| role_object.role)
                            .collect();
                        publisher_roles.contains(&String::from("funder"))
                    }
                    None => {
                        eprintln!("vertex was not publisher");
                        false
                    }
                },
                unhandled => unreachable!("{:?}", unhandled),
            };

            (ctx, can_coerce)
        });

        Box::new(iterator)
    }
}
