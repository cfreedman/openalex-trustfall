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
    created_data: String,
    display_name: String,
    id: String,
    ids: Vec<String>,
    updated_data: String,
}
// OpenAlexWork specific properties' types
struct OpenAlexWork {
    object: OpenAlexObject,
    abstract: String,
    doi: String,
    publication_date: String,
    publication_year: usize,
    title: String,
    is_oa: bool,
    license: String,
    url: String
}

struct OpenAlexAuthor {
    object: OpenAlexObject,
    display_name_alternatives: Vec<String>,
    last_known_institution: String,
    works_count: usize
}

struct OpenAlexConcept {
    object: OpenAlexObject,
    description: String,
    level: usize,
    works_count: usize
}

struct OpenAlexSource {
    object: OpenAlexObject,
    alternative_titles: Vec<String>,
    country_code: String,
    homepage_url: String,
    host_organization: String,
    host_organization_name: String,
    is_oa: bool,
    works_count: usize
}

struct OpenAlexInstitution {
    object: OpenAlexObject,
    country_code: String,
    homepage_url: String,
    ror: String,
    works_count: usize
}

struct OpenAlexPublisher {
    object: OpenAlexObject,
    alternative_titles: Vec<String>,
    hierarchy_level: usize,
    works_count: usize
}

struct OpenAlexFunder {
    object: OpenAlexObject,
    alternative_titles: Vec<String>,
    country_code: String,
    grants_count: usize,
    homepage_url: String,
    works_count: usize
}