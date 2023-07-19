use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Vertex {
    Work(Work),
    Author(Author),
    Concept(Concept),
    Source(Source),
    Institution(Institution),
    Publisher(Publisher),
    Funder(Funder)
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

    pub fn as_work(&self) -> Option<&Work>{
        match self {
            Vertex::Work(work) => Some(work),
            _ => None,
        }
    }

    pub fn as_author(&self) -> Option<&Author>{
        match self {
            Vertex::Author(author) => Some(author),
            _ => None,
        }
    }

    pub fn as_concept(&self) -> Option<&Concept>{
        match self {
            Vertex::Concept(concept) => Some(concept),
            _ => None,
        }
    }

    pub fn as_source(&self) -> Option<&Source>{
        match self {
            Vertex::Source(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_institution(&self) -> Option<&Institution>{
        match self {
            Vertex::Institution(institution) => Some(institution),
            _ => None,
        }
    }

    pub fn as_publisher(&self) -> Option<&Publisher>{
        match self {
            Vertex::Publisher(publisher) => Some(publisher),
            _ => None,
        }
    }

    pub fn as_funder(&self) -> Option<&Funder>{
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
    pub created_data: String,
    pub display_name: String,
    pub id: String,
    pub ids: Vec<String>,
    pub updated_data: String,
}
// OpenAlexWork structs
#[derive(Clone, Debug, Deserialize)]
pub struct Work{
    pub object: OpenAlexObject,
    pub abstract_text: String,
    pub authorships: Vec<Authorship>,
    pub apc_payment: Payment,
    pub best_oa_location: Location,
    pub biblio: Biblio,
    pub cited_by_api_url: String,
    pub concepts: Vec<DehydratedConcept>,
    pub corresponding_author_ids: Vec<String>,
    pub corresponding_institution_ids: Vec<String>,
    pub doi: String,
    pub grants: Vec<Grant>,
    pub is_paratext: bool,
    pub is_retracted: bool,
    pub language: String,
    pub locations: Vec<Location>,
    pub mesh: Vec<Mesh>,
    pub ngrams_url: String,
    pub open_access: OpenAccess,
    pub primary_location: Location,
    pub publication_date: String,
    pub publication_year: u16,
    pub referenced_works: Vec<String>,
    pub related_works: Vec<String>,
    pub title: String,
    pub ttype: String,
    pub is_oa: bool,
    pub license: String,
    pub url: String,
    pub version: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Authorship{
    pub author_position: String,
    pub author: DehydratedAuthor,
    pub institutions: Vec<DehydratedInstitution>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Payment {
    pub price: u32,
    pub currency: String,
    pub provenance: String,
    pub price_usd: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Biblio {
    pub volume: String,
    pub issue: String,
    pub first_page: String,
    pub last_page: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct YearCount {
    pub year: u16,
    pub cited_by_count: u32
}

#[derive(Clone, Debug, Deserialize)]
pub struct Grant {
    pub funder: String,
    pub funder_display_name: String,
    pub award_id: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Location{
    pub is_oa: bool,
    pub landing_page_url: String,
    pub license: String,
    pub source: DehydratedSource,
    pub pdf_url: String,
    pub version: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mesh {
    pub descriptor_ui: String,
    pub descriptor_name: String,
    pub qualifier_ui: String,
    pub qualifier_name: String,
    pub is_major_topic: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAccess {
    pub is_oa: bool,
    pub oa_status: String,
    pub oa_url: String,
    pub any_repository_has_fulltext: bool
}

// OpenAlexAuthor structs
#[derive(Clone, Debug)]
pub struct Author{
    pub object: OpenAlexObject,
    pub display_name_alternatives: Vec<String>,
    pub last_known_institution: DehydratedInstitution,
    pub orcid: String,
    pub summary_stats: SummaryStats,
    pub works_api_url: String,
    pub works_count: u32
}

#[derive(Clone, Debug)]
pub struct SummaryStats {
    two_year_mean_citedness: f64,
    h_index: u32,
    i10_index: u32
}

#[derive(Clone, Debug)]
pub struct DehydratedAuthor {
    pub id: String,
    pub display_name: String,
    pub orcid: String
}

// Concept structs
#[derive(Clone, Debug)]
pub struct Concept{
    pub ancestors: Vec<DehydratedConcept>,
    pub object: OpenAlexObject,
    pub description: String,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub level: u16,
    pub related_concepts: Vec<DehydratedConcept>,
    pub summary_stats: SummaryStats,
    pub wikidata: String,
    pub works_api_url: String,
    pub works_count: u32
}

#[derive(Clone, Debug)]
pub struct DehydratedConcept {
    pub id: String,
    pub wikidata: String,
    pub display_name: String,
    pub level: u16,
    pub score: f64
}

 // Source structs
 #[derive(Clone, Debug)]
pub struct Source{
    pub object: OpenAlexObject,
    pub abreviated_title: String,
    pub alternative_titles: Vec<String>,
    pub apc_payment: Vec<Price>,
    pub apc_usd: u16,
    pub country_code: String,
    pub homepage_url: String,
    pub host_organization: String,
    pub host_organization_lineage: Vec<String>,
    pub host_organization_name: String,
    pub is_in_doaj: bool,
    pub is_oa: bool,
    pub issn: Vec<String>,
    pub issn_l: String,
    pub societies: Vec<Society>,
    pub summary_stats: SummaryStats,
    pub ttype: String,
    pub works_api_url: String,
    pub works_count: u32
}

#[derive(Clone, Debug)]
pub struct Price {
    pub price: u16,
    pub currency: String
}

#[derive(Clone, Debug)]
pub struct Society {
    pub url: String,
    pub organization: String
}

#[derive(Clone, Debug)]
pub struct DehydratedSource{
    pub id: String,
    pub display_name: String,
    pub issn_l: String,
    pub issn: Vec<String>,
    pub host_organization: String,
    pub ttype: String
}

// Institution structs
#[derive(Clone, Debug)]
pub struct Institution{
    pub object: OpenAlexObject,
    pub associated_institutions: Vec<DehydratedInstitution>,
    pub display_name_alternatives: Vec<String>,
    pub country_code: String,
    pub geo: Geo,
    pub homepage_url: String,
    pub repositories: Vec<DehydratedSource>,
    pub roles: Vec<Role>,
    pub ror: String,
    pub summary_stats: SummaryStats,
    pub ttype: String,
    pub works_api_url: String,
    pub works_count: u32
}

#[derive(Clone, Debug)]
pub struct DehydratedInstitution {
    pub id: String,
    pub display_name: String,
    pub ror: String,
    pub country_code: String,
    pub ttype: String
}

#[derive(Clone, Debug)]
pub struct Role {
    pub role: String,
    pub id: String,
    pub works_count: u32
}

#[derive(Clone, Debug)]
pub struct Geo {
    pub city: String,
    pub geonames_city_id: String,
    pub region: String,
    pub country_code: String,
    pub country: String,
    pub latitude: i32,
    pub longitude: i32
}

// Publisher structs
#[derive(Clone, Debug)]
pub struct Publisher{
    pub object: OpenAlexObject,
    pub alternative_titles: Vec<String>,
    pub country_codes: Vec<String>,
    pub hierarchy_level: u16,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub lineage: Vec<String>,
    pub parent_publisher: String,
    pub roles: Vec<Role>,
    pub sources_api_url: String,
    pub summary_stats: SummaryStats,
    pub works_count: u32
}

// Funder structs
#[derive(Clone, Debug)]
pub struct Funder{
    pub object: OpenAlexObject,
    pub alternative_titles: Vec<String>,
    pub country_code: String,
    pub description: String,
    pub grants_count: u32,
    pub homepage_url: String,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub roles: Vec<Role>,
    pub summary_stats: SummaryStats,
    pub works_count: u32
}