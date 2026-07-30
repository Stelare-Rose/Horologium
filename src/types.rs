use chrono::{DateTime, NaiveDate, Utc};

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

pub struct Color(String);
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

