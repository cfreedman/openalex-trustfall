pub enum Vertex {
    OpenAlexWork,
    OpenAlexAuthor,
    OpenAlexConcept,
    OpenAlexSource,
    OpenAlexInstitution,
    OpenAlexPublisher,
    OpenAlexFunder
}
// Basic object with properties shared by all OpenAlex entities
struct OpenAlexObject {
    cited_by_count: usize,
    counts_by_year: &[YearCount],
    created_data: String,
    display_name: String,
    id: String,
    ids: &[String],
    updated_data: String,
}
// OpenAlexWork structs
struct Work {
    object: OpenAlexObject,
    abstract: String,
    authorships: &[Authorships],
    apc_payment: Payment,
    best_oa_location: Location,
    biblio: Biblio,
    cited_by_api_url: String,
    concepts: &[DehydratedConcept],
    corresponding_author_ids: &[String],
    corresponding_institution_ids: &[String],
    doi: String,
    grants: &[Grant],
    is_paratext: bool,
    is_retracted: bool,
    language: String,
    locations: &[Location],
    mesh: &[Mesh],
    ngrams_url: String,
    open_access: OpenAccess,
    primary_location: Location,
    publication_date: String,
    publication_year: usize,
    referenced_works: &[String],
    related_works: &[String],
    title: String,
    ttype: String,
    is_oa: bool,
    license: String,
    url: String,
    version: String
}

struct Authorship {
    author_position: String,
    author: DehydratedAuthor,
    institutions: &[DehydratedInstitution]
}

struct Payment {
    price: usize,
    currency: String,
    provenance: String,
    price_usd: usize,
}

struct Biblio {
    volume: String,
    issue: String,
    first_page: String,
    last_page: String
}

struct YearCount {
    year: usize,
    cited_by_count: usize
}

struct Grant {
    funder: String,
    funder_display_name: String,
    award_id: String
}

struct Location {
    is_oa: bool,
    landing_page_url: String,
    license: String,
    source: DehydratedSource,
    pdf_url: String,
    version: String
}

struct Mesh {
    descriptor_ui: String,
    descriptor_name: String,
    qualifier_ui: String,
    qualifier_name: String,
    is_major_topic: bool
}

struct OpenAccess {
    is_oa: bool,
    oa_status: String,
    oa_url: String,
    any_repository_has_fulltext: bool
}

// OpenAlexAuthor structs
struct Author {
    object: OpenAlexObject,
    display_name_alternatives: &[String],
    last_known_institution: DehydratedInstitution,
    orcid: String,
    summary_stats: SummaryStats,
    works_api_url: String,
    works_count: usize
}

struct SummaryStats {
    two_year_mean_citedness: f64,
    h_index: usize,
    i10_index: usize
}

struct DehydratedAuthor {
    id: String,
    display_name: String,
    orcid: String
}

// Concept structs
struct Concept {
    ancestors: &[DehydratedConcept],
    object: OpenAlexObject,
    description: String,
    image_thumbnail_url: String,
    image_url: String,
    level: usize,
    related_concepts: &[DehydratedConcept],
    summary_stats: SummaryStats,
    wikidata: String,
    works_api_url: &[String],
    works_count: usize
}

struct DehydratedConcept {
    id: String,
    wikidata: String,
    display_name: String,
    level: usize,
    score: f64
}

 // Source structs
struct Source {
    object: OpenAlexObject,
    abreviated_title: String,
    alternative_titles: &[String],
    apc_payment: &[Price],
    apc_usd: u16,
    country_code: String,
    homepage_url: String,
    host_organization: String,
    host_organization_lineage: &[String],
    host_organization_name: String,
    is_in_doaj: bool,
    is_oa: bool,
    issn: &[String],
    issn_l: String,
    societies: &[Society],
    summary_stats: SummaryStats,
    ttype: String,
    works_api_url: &[String],
    works_count: usize
}

struct Price {
    price: u16,
    currency: String
}

struct Society {
    url: String,
    organization: String
}

struct DehydratedSource {
    id: String,
    display_name: String,
    issn_l: String,
    issn: &[String],
    host_organization: String,
    ttype: String
}

// Institution structs
struct Institution {
    object: OpenAlexObject,
    associated_institutions: &[DehydratedInstitution],
    display_name_alternatives: &[String],
    country_code: String,
    geo: Geo,
    homepage_url: String,
    repositories: &[DehydratedSource],
    roles: &[Role],
    ror: String,
    summary_stats: SummaryStats,
    ttype: String,
    works_api_url: &[String],
    works_count: usize
}

struct DehydratedInstitution {
    id: String,
    display_name: String,
    ror: String,
    country_code: String,
    ttype: String
}

struct Role {
    role: String,
    id: String,
    works_count: usize
}

struct Geo {
    city: String,
    geonames_city_id: String,
    region: String,
    country_code: String,
    country: String,
    latitude: i32,
    longitude: i32
}

// Publisher structs
struct Publisher {
    object: OpenAlexObject,
    alternative_titles: &[String],
    country_codes: &[String],
    hierarchy_level: usize,
    image_thumbnail_url: String,
    image_url: String,
    lineage: &[string],
    parent_publisher: String,
    roles: &[Role],
    sources_api_url: &[String],
    summary_stats: SummaryStats,
    works_count: usize
}

// Funder structs
struct Funder {
    object: OpenAlexObject,
    alternative_titles: &[String],
    country_code: String,
    description: String,
    grants_count: usize,
    homepage_url: String,
    image_thumbnail_url: String,
    image_url: String,
    roles: &[Role],
    summary_stats: SummaryStats,
    works_count: usize
}