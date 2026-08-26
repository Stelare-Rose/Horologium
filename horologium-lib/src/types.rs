use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

mod record;
mod tags;
mod project;

// Do not @ me about bad encapsulation. fix it yourself
pub struct RecordId(pub String);
pub struct TagId { pub id: String, pub name: String }
pub struct ProjectId { pub id: String, pub name: String }
pub struct Record {
    pub id: RecordId,
    pub start: DateTime<Utc>,
    pub event: Event,
    pub body: Option<String>
}
pub enum Event {
    Active {
        name: String,
        tags: Vec<TagId>,
        project: Option<ProjectId>,
    },
    Inactive {
        name: Option<String>,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Color(Colorscheme);
impl Color {
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Vec<Color>,
}
pub enum ProjectState {
    Active,
    Done {
        completed_date: NaiveDate 
    }
}
pub struct Project {
    pub id: String,
    pub name: String,
    pub state: ProjectState,
    pub color: Vec<Color>,
    pub body: Option<String>
}
#[derive(Hash)]
pub struct Fingerprint {
    pub size: u64,
    pub mtime: i64,
    pub mtime_nsec: i64,
    pub ctime: i64,
    pub ctime_nsec: i64,
}

#[derive(Display, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Colorscheme {
    Strawberry,
    Orange,
    Lemon,
    Leaf,
    Mint,
    Sky,
    Blueberry,
    Grape,
    Plum,
    Lavender,
    Lilac,
    Pink
}
