#[derive(Clone, Debug)]
pub enum Vertex<'a> {
    Work(Work<'a>),
    Author(Author<'a>),
    Concept(Concept<'a>),
    Source(Source<'a>),
    Institution(Institution<'a>),
    Publisher(Publisher<'a>),
    Funder(Funder<'a>)
}

impl<'a> Vertex<'a> {

    pub fn typename(&self) -> &'a str {
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

    pub fn as_work(&self) -> Option<&Work<'a>>{
        match self {
            Vertex::Work(work) => Some(work),
            _ => None,
        }
    }

    pub fn as_author(&self) -> Option<&Author<'a>>{
        match self {
            Vertex::Author(author) => Some(author),
            _ => None,
        }
    }

    pub fn as_concept(&self) -> Option<&Concept<'a>>{
        match self {
            Vertex::Concept(concept) => Some(concept),
            _ => None,
        }
    }

    pub fn as_source(&self) -> Option<&Source<'a>>{
        match self {
            Vertex::Source(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_institution(&self) -> Option<&Institution<'a>>{
        match self {
            Vertex::Institution(institution) => Some(institution),
            _ => None,
        }
    }

    pub fn as_publisher(&self) -> Option<&Publisher<'a>>{
        match self {
            Vertex::Publisher(publisher) => Some(publisher),
            _ => None,
        }
    }

    pub fn as_funder(&self) -> Option<&Funder<'a>>{
        match self {
            Vertex::Funder(funder) => Some(funder),
            _ => None,
        }
    }
}
// Basic object with properties shared by all OpenAlex entities
#[derive(Clone, Debug)]
pub struct OpenAlexObject<'a>{
    pub cited_by_count: u32,
    pub counts_by_year: &'a [YearCount],
    pub created_data: String,
    pub display_name: String,
    pub id: String,
    pub ids: &'a [String],
    pub updated_data: String,
}
// OpenAlexWork structs
#[derive(Clone, Debug)]
pub struct Work<'a>{
    pub object: OpenAlexObject<'a>,
    pub abstract_text: String,
    pub authorships: &'a [Authorship<'a>],
    pub apc_payment: Payment,
    pub best_oa_location: Location<'a>,
    pub biblio: Biblio,
    pub cited_by_api_url: String,
    pub concepts: &'a [DehydratedConcept],
    pub corresponding_author_ids: &'a [String],
    pub corresponding_institution_ids: &'a [String],
    pub doi: String,
    pub grants: &'a [Grant],
    pub is_paratext: bool,
    pub is_retracted: bool,
    pub language: String,
    pub locations: &'a [Location<'a>],
    pub mesh: &'a [Mesh],
    pub ngrams_url: String,
    pub open_access: OpenAccess,
    pub primary_location: Location<'a>,
    pub publication_date: String,
    pub publication_year: u16,
    pub referenced_works: &'a [String],
    pub related_works: &'a [String],
    pub title: String,
    pub ttype: String,
    pub is_oa: bool,
    pub license: String,
    pub url: String,
    pub version: String
}

#[derive(Clone, Debug)]
pub struct Authorship<'a>{
    pub author_position: String,
    pub author: DehydratedAuthor,
    pub institutions: &'a [DehydratedInstitution]
}

#[derive(Clone, Debug)]
pub struct Payment {
    pub price: u32,
    pub currency: String,
    pub provenance: String,
    pub price_usd: u32,
}

#[derive(Clone, Debug)]
pub struct Biblio {
    pub volume: String,
    pub issue: String,
    pub first_page: String,
    pub last_page: String
}

#[derive(Clone, Debug)]
pub struct YearCount {
    pub year: u16,
    pub cited_by_count: u32
}

#[derive(Clone, Debug)]
pub struct Grant {
    pub funder: String,
    pub funder_display_name: String,
    pub award_id: String
}

#[derive(Clone, Debug)]
pub struct Location<'a>{
    pub is_oa: bool,
    pub landing_page_url: String,
    pub license: String,
    pub source: &'a DehydratedSource<'a>,
    pub pdf_url: String,
    pub version: String
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub descriptor_ui: String,
    pub descriptor_name: String,
    pub qualifier_ui: String,
    pub qualifier_name: String,
    pub is_major_topic: bool
}

#[derive(Clone, Debug)]
pub struct OpenAccess {
    pub is_oa: bool,
    pub oa_status: String,
    pub oa_url: String,
    pub any_repository_has_fulltext: bool
}

// OpenAlexAuthor structs
#[derive(Clone, Debug)]
pub struct Author<'a>{
    pub object: OpenAlexObject<'a>,
    pub display_name_alternatives: &'a [String],
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
pub struct Concept<'a>{
    pub ancestors: &'a [DehydratedConcept],
    pub object: OpenAlexObject<'a>,
    pub description: String,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub level: u16,
    pub related_concepts: &'a [DehydratedConcept],
    pub summary_stats: SummaryStats,
    pub wikidata: String,
    pub works_api_url: &'a [String],
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
pub struct Source<'a>{
    pub object: OpenAlexObject<'a>,
    pub abreviated_title: String,
    pub alternative_titles: &'a [String],
    pub apc_payment: &'a [Price],
    pub apc_usd: u16,
    pub country_code: String,
    pub homepage_url: String,
    pub host_organization: String,
    pub host_organization_lineage: &'a [String],
    pub host_organization_name: String,
    pub is_in_doaj: bool,
    pub is_oa: bool,
    pub issn: &'a [String],
    pub issn_l: String,
    pub societies: &'a [Society],
    pub summary_stats: SummaryStats,
    pub ttype: String,
    pub works_api_url: &'a [String],
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
pub struct DehydratedSource<'a>{
    pub id: String,
    pub display_name: String,
    pub issn_l: String,
    pub issn: &'a [String],
    pub host_organization: String,
    pub ttype: String
}

// Institution structs
#[derive(Clone, Debug)]
pub struct Institution<'a>{
    pub object: OpenAlexObject<'a>,
    pub associated_institutions: &'a [DehydratedInstitution],
    pub display_name_alternatives: &'a [String],
    pub country_code: String,
    pub geo: Geo,
    pub homepage_url: String,
    pub repositories: &'a [DehydratedSource<'a>],
    pub roles: &'a [Role],
    pub ror: String,
    pub summary_stats: SummaryStats,
    pub ttype: String,
    pub works_api_url: &'a [String],
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
pub struct Publisher<'a>{
    pub object: OpenAlexObject<'a>,
    pub alternative_titles: &'a [String],
    pub country_codes: &'a [String],
    pub hierarchy_level: u16,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub lineage: &'a [String],
    pub parent_publisher: String,
    pub roles: &'a [Role],
    pub sources_api_url: &'a [String],
    pub summary_stats: SummaryStats,
    pub works_count: u32
}

// Funder structs
#[derive(Clone, Debug)]
pub struct Funder<'a>{
    pub object: OpenAlexObject<'a>,
    pub alternative_titles: &'a [String],
    pub country_code: String,
    pub description: String,
    pub grants_count: u32,
    pub homepage_url: String,
    pub image_thumbnail_url: String,
    pub image_url: String,
    pub roles: &'a [Role],
    pub summary_stats: SummaryStats,
    pub works_count: u32
}