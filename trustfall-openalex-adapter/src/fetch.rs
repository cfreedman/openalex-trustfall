use crate::vertex::{
    Author, Concept, FilteredVertices, Funder, Institution, Publisher, Source, Vertex, VertexKind,
    Work,
};
use lazy_static::lazy_static;
use reqwest::{
    blocking::Client,
    header::{ACCEPT, HOST, USER_AGENT},
    Error,
};

lazy_static! {
    static ref OPEN_ALEX_CLIENT: Client = Client::new();
}

pub fn fetch_vertex(url: String, kind: VertexKind) -> Result<Vertex, Error> {
    let response = OPEN_ALEX_CLIENT
        .get(url.clone())
        .header(USER_AGENT, "curl/7.81.0")
        .header(HOST, "api.openalex.org")
        .header(ACCEPT, "*/*")
        .send()?;
    println!("{:?}", response.text());
    let request = OPEN_ALEX_CLIENT
        .get(url.clone())
        .header(USER_AGENT, "curl/7.81.0")
        .header(HOST, "api.openalex.org")
        .header(ACCEPT, "*/*");
    let json_response = request.send()?;
    match kind {
        VertexKind::Work => Ok(Vertex::Work(json_response.json::<Work>()?)),
        VertexKind::Author => Ok(Vertex::Author(json_response.json::<Author>()?)),
        VertexKind::Source => Ok(Vertex::Source(json_response.json::<Source>()?)),
        VertexKind::Concept => Ok(Vertex::Concept(json_response.json::<Concept>()?)),
        VertexKind::Institution => Ok(Vertex::Institution(json_response.json::<Institution>()?)),
        VertexKind::Publisher => Ok(Vertex::Publisher(json_response.json::<Publisher>()?)),
        VertexKind::Funder => Ok(Vertex::Funder(json_response.json::<Funder>()?)),
    }
}

pub fn fetch_vertices(url: String, kind: VertexKind) -> Result<Vec<Vertex>, Error> {
    let json_response = OPEN_ALEX_CLIENT.get(url).send()?;

    match kind {
        VertexKind::Work => Ok(json_response
            .json::<FilteredVertices<Work>>()?
            .results
            .into_iter()
            .map(|work| Vertex::Work(work))
            .collect::<Vec<_>>()),
        VertexKind::Author => Ok(json_response
            .json::<FilteredVertices<Author>>()?
            .results
            .into_iter()
            .map(|author| Vertex::Author(author))
            .collect::<Vec<_>>()),
        VertexKind::Source => Ok(json_response
            .json::<FilteredVertices<Source>>()?
            .results
            .into_iter()
            .map(|source| Vertex::Source(source))
            .collect::<Vec<_>>()),
        VertexKind::Concept => Ok(json_response
            .json::<FilteredVertices<Concept>>()?
            .results
            .into_iter()
            .map(|concept| Vertex::Concept(concept))
            .collect::<Vec<_>>()),
        VertexKind::Institution => Ok(json_response
            .json::<FilteredVertices<Institution>>()?
            .results
            .into_iter()
            .map(|institution| Vertex::Institution(institution))
            .collect::<Vec<_>>()),
        VertexKind::Publisher => Ok(json_response
            .json::<FilteredVertices<Publisher>>()?
            .results
            .into_iter()
            .map(|publisher| Vertex::Publisher(publisher))
            .collect::<Vec<_>>()),
        VertexKind::Funder => Ok(json_response
            .json::<FilteredVertices<Funder>>()?
            .results
            .into_iter()
            .map(|funder| Vertex::Funder(funder))
            .collect::<Vec<_>>()),
    }
}
