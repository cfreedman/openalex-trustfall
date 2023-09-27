use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Vertex {
    Work(Work),
    Author(Author),
    Concept(Concept),
    Source(Source),
    Institution(Institution),
    Publisher(Publisher),
    Funder(Funder),
}

pub enum VertexKind {
    Work,
    Author,
    Concept,
    Source,
    Institution,
    Publisher,
    Funder,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FilteredVertices<T> {
    pub results: Vec<T>,
    meta: MetaData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MetaData {
    count: u32,
    db_response_time_ms: u32,
    page: u32,
    per_page: u32,
}

impl Vertex {
    pub fn typename<'a>(&self) -> &'a str {
        match self {
            Vertex::Work(..) => "Work",
            Vertex::Author(..) => "Author",
            Vertex::Concept(..) => "Concept",
            Vertex::Source(..) => "Source",
            Vertex::Institution(..) => "Institution",
            Vertex::Publisher(..) => "Publisher",
            Vertex::Funder(..) => "Funder",
        }
    }

    pub fn as_work(&self) -> Option<&Work> {
        match self {
            Vertex::Work(work) => Some(work),
            _ => None,
        }
    }

    pub fn as_author(&self) -> Option<&Author> {
        match self {
            Vertex::Author(author) => Some(author),
            _ => None,
        }
    }

    pub fn as_concept(&self) -> Option<&Concept> {
        match self {
            Vertex::Concept(concept) => Some(concept),
            _ => None,
        }
    }

    pub fn as_source(&self) -> Option<&Source> {
        match self {
            Vertex::Source(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_institution(&self) -> Option<&Institution> {
        match self {
            Vertex::Institution(institution) => Some(institution),
            _ => None,
        }
    }

    pub fn as_publisher(&self) -> Option<&Publisher> {
        match self {
            Vertex::Publisher(publisher) => Some(publisher),
            _ => None,
        }
    }

    pub fn as_funder(&self) -> Option<&Funder> {
        match self {
            Vertex::Funder(funder) => Some(funder),
            _ => None,
        }
    }
}
// Basic object with properties shared by all OpenAlex entities
#[derive(Clone, Debug, Deserialize)]
pub struct OpenAlexObject {
    pub cited_by_count: u32,
    pub counts_by_year: Vec<YearCount>,
    pub created_date: String,
    pub display_name: String,
    pub id: String,
    pub ids: IDObject, // Fix
    pub updated_date: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IDObject {
    pub doi: Option<String>,
    pub mag: Option<String>,
    pub openalex: Option<String>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
}
// OpenAlexWork structs
#[derive(Clone, Debug, Deserialize)]
pub struct Work {
    #[serde(flatten)]
    pub object: OpenAlexObject,

    pub abstract_inverted_index: Option<String>,
    pub authorships: Vec<Authorship>,
    pub apc_list: Option<Payment>,
    pub apc_paid: Option<Payment>,
    pub best_oa_location: Option<Location>,
    pub biblio: Option<Biblio>,
    pub cited_by_api_url: String,
    pub concepts: Vec<DehydratedConcept>,
    pub corresponding_author_ids: Vec<String>,
    pub corresponding_institution_ids: Vec<String>,
    pub doi: Option<String>,
    pub grants: Vec<Grant>,
    pub institutions_distinct_count: Option<u16>,
    pub is_oa: Option<bool>,
    pub is_paratext: Option<bool>,
    pub is_retracted: Option<bool>,
    pub language: Option<String>,
    pub license: Option<String>,
    pub locations: Vec<Location>,
    pub locations_count: Option<u16>,
    pub mesh: Vec<Mesh>,
    pub ngrams_url: Option<String>,
    pub open_access: Option<OpenAccess>,
    pub primary_location: Option<Location>,
    pub publication_date: Option<String>,
    pub publication_year: Option<u16>,
    pub referenced_works: Vec<String>,
    pub related_works: Vec<String>,
    pub sustainable_development_goals: Vec<SustainableObject>,
    pub title: Option<String>,

    #[serde(rename(deserialize = "type"))]
    pub ttype: Option<String>,

    pub type_crossref: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Authorship {
    pub author_position: Option<String>,
    pub author: DehydratedAuthor,
    pub institutions: Vec<DehydratedInstitution>,
    pub countries: Vec<String>,
    pub is_corresponding: Option<bool>,
    pub raw_affiliation_string: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Payment {
    pub value: Option<u32>,
    pub currency: Option<String>,
    pub provenance: Option<String>,
    pub value_usd: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Biblio {
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct YearCount {
    pub year: u16,
    pub works_count: Option<u32>,
    pub cited_by_count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Grant {
    pub funder: String,
    pub funder_display_name: String,
    pub award_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub is_accepted: Option<bool>,
    pub is_oa: Option<bool>,
    pub is_published: Option<bool>,
    pub landing_page_url: Option<String>,
    pub license: Option<String>,
    pub source: Option<DehydratedSource>,
    pub pdf_url: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mesh {
    pub descriptor_ui: Option<String>,
    pub descriptor_name: Option<String>,
    pub qualifier_ui: Option<String>,
    pub qualifier_name: Option<String>,
    pub is_major_topic: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAccess {
    pub is_oa: Option<bool>,
    pub oa_status: Option<String>,
    pub oa_url: Option<String>,
    pub any_repository_has_fulltext: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SustainableObject {
    id: Option<String>,
    display_name: Option<String>,
    score: Option<u16>,
}

// OpenAlexAuthor structs
#[derive(Clone, Debug, Deserialize)]
pub struct Author {
    pub object: OpenAlexObject,
    pub display_name_alternatives: Vec<String>,
    pub last_known_institution: Option<DehydratedInstitution>,
    pub orcid: Option<String>,
    pub summary_stats: Option<SummaryStats>,
    pub works_api_url: String,
    pub works_count: Option<u32>,
    // x_concepts?
}

#[derive(Clone, Debug, Deserialize)]
pub struct SummaryStats {
    pub two_year_mean_citedness: Option<f64>,
    pub h_index: Option<u32>,
    pub i10_index: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DehydratedAuthor {
    pub id: String,
    pub display_name: String,
    pub orcid: Option<String>,
}

// Concept structs
#[derive(Clone, Debug, Deserialize)]
pub struct Concept {
    pub ancestors: Vec<DehydratedConcept>,
    pub object: OpenAlexObject,
    pub description: Option<String>,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub level: Option<u16>,
    pub related_concepts: Vec<DehydratedConcept>,
    pub summary_stats: Option<SummaryStats>,
    pub wikidata: Option<String>,
    pub works_api_url: String,
    pub works_count: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DehydratedConcept {
    pub id: String,
    pub wikidata: Option<String>,
    pub display_name: String,
    pub level: u16,
    pub score: f64, // Fix
}

// Source structs
#[derive(Clone, Debug, Deserialize)]
pub struct Source {
    pub object: OpenAlexObject,
    pub abreviated_title: Option<String>,
    pub alternative_titles: Vec<String>,
    pub apc_payment: Vec<Price>,
    pub apc_usd: Option<u16>,
    pub country_code: Option<String>,
    pub homepage_url: Option<String>,
    pub host_organization: Option<String>,
    pub host_organization_lineage: Vec<String>,
    pub host_organization_name: Option<String>,
    pub is_in_doaj: Option<bool>,
    pub is_oa: Option<bool>,
    pub issn: Vec<String>,
    pub issn_l: Option<String>,
    pub societies: Vec<Society>,
    pub summary_stats: Option<SummaryStats>,

    #[serde(rename(deserialize = "type"))]
    pub ttype: Option<String>,

    pub works_api_url: String,
    pub works_count: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Price {
    pub price: u16,
    pub currency: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Society {
    pub url: String,
    pub organization: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DehydratedSource {
    // Fix
    pub id: String,
    pub display_name: String,
    pub issn_l: Option<String>,
    pub issn: Vec<String>,
    pub host_organization: Option<String>,

    #[serde(rename(deserialize = "type"))]
    pub ttype: Option<String>,
}

// Institution structs
#[derive(Clone, Debug, Deserialize)]
pub struct Institution {
    pub object: OpenAlexObject,
    pub associated_institutions: Vec<DehydratedInstitution>,
    pub display_name_alternatives: Vec<String>,
    pub display_name_acronyms: Vec<String>,
    pub country_code: Option<String>,
    pub geo: Option<Geo>,
    pub homepage_url: String,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub repositories: Vec<DehydratedSource>,
    pub roles: Vec<Role>,
    pub ror: Option<String>,
    pub summary_stats: Option<SummaryStats>,

    #[serde(rename(deserialize = "type"))]
    pub ttype: Option<String>,

    pub works_api_url: String,
    pub works_count: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DehydratedInstitution {
    // Fix
    pub id: String,
    pub display_name: String,
    pub ror: Option<String>,
    pub country_code: Option<String>,

    #[serde(rename(deserialize = "type"))]
    pub ttype: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Role {
    pub role: String,
    pub id: String,
    pub works_count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Geo {
    pub city: Option<String>,
    pub geonames_city_id: Option<String>,
    pub region: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<i32>,
    pub longitude: Option<i32>,
}

// Publisher structs
#[derive(Clone, Debug, Deserialize)]
pub struct Publisher {
    pub object: OpenAlexObject,
    pub alternative_titles: Vec<String>,
    pub country_codes: Vec<String>,
    pub hierarchy_level: Option<u16>,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub lineage: Vec<String>,
    pub parent_publisher: Option<String>,
    pub roles: Vec<Role>,
    pub sources_api_url: String,
    pub summary_stats: Option<SummaryStats>,
    pub works_count: Option<u32>,
}

// Funder structs
#[derive(Clone, Debug, Deserialize)]
pub struct Funder {
    pub object: OpenAlexObject,
    pub alternative_titles: Vec<String>,
    pub country_code: Option<String>,
    pub description: Option<String>,
    pub grants_count: Option<u32>,
    pub homepage_url: Option<String>,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub roles: Vec<Role>,
    pub summary_stats: Option<SummaryStats>,
    pub works_count: Option<u32>,
}
