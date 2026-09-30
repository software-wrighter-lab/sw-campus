use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(transparent)]
pub struct PlaceId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PlaceKind {
    Campus,
    Building,
    Wing,
    Exhibit,
    Demo,
    Site,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Status {
    Open,
    Placeholder,
    ComingSoon,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Link {
    pub label: String,
    pub url: String,
    pub kind: LinkKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum LinkKind {
    Run,
    Source,
    Docs,
    Related,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Place {
    pub id: PlaceId,
    pub kind: PlaceKind,
    pub title: String,
    pub tagline: String,
    pub summary: String,
    pub status: Status,
    pub children: Vec<PlaceId>,
    pub scene: Option<String>,
    pub links: Vec<Link>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub example_queries: Vec<String>,
}
