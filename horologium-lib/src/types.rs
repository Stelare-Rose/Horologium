use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

mod project;
mod record;
mod tags;

// Do not @ me about bad encapsulation. fix it yourself
#[derive(Serialize, Deserialize)]
pub struct RecordId(pub String);
#[derive(Serialize, Deserialize)]
pub struct TagId {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct ProjectId {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: RecordId,
    pub start: DateTime<Utc>,
    pub event: Event,
    pub body: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum Event {
    Active {
        name: String,
        tags: Vec<TagId>,
        project: Option<ProjectId>,
    },
    Inactive {
        name: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Color(pub Colorscheme);
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
    Done { completed_date: NaiveDate },
}
pub struct Project {
    pub id: String,
    pub name: String,
    pub state: ProjectState,
    pub color: Vec<Color>,
    pub body: Option<String>,
}
#[derive(Hash)]
pub struct Fingerprint {
    pub size: u64,
    pub mtime: i64,
    pub mtime_nsec: i64,
    pub ctime: i64,
    pub ctime_nsec: i64,
}

#[derive(Display, EnumString, EnumIter, Serialize, Deserialize)]
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
    Pink,
}

#[derive(Clone)]
pub enum Mode {
    Light,
    Dark,
}

impl Colorscheme {
    pub fn hex(&self, mode: &Mode) -> &'static str {
        match (self, mode) {
            (Colorscheme::Strawberry, Mode::Light) => "#cf8282",
            (Colorscheme::Strawberry, Mode::Dark) => "#cf8282",

            (Colorscheme::Orange, Mode::Light) => "#d4a266",
            (Colorscheme::Orange, Mode::Dark) => "#d4a266",

            (Colorscheme::Lemon, Mode::Light) => "#e3cd7f",
            (Colorscheme::Lemon, Mode::Dark) => "#e3cd7f",

            (Colorscheme::Leaf, Mode::Light) => "#9ccc96",
            (Colorscheme::Leaf, Mode::Dark) => "#9ccc96",

            (Colorscheme::Mint, Mode::Light) => "#99cfac",
            (Colorscheme::Mint, Mode::Dark) => "#99cfac",

            (Colorscheme::Sky, Mode::Light) => "#90cdde",
            (Colorscheme::Sky, Mode::Dark) => "#90cdde",

            (Colorscheme::Blueberry, Mode::Light) => "#9bb0de",
            (Colorscheme::Blueberry, Mode::Dark) => "#9bb0de",

            (Colorscheme::Grape, Mode::Light) => "#9b98d6",
            (Colorscheme::Grape, Mode::Dark) => "#9b98d6",

            (Colorscheme::Plum, Mode::Light) => "#aa99d1",
            (Colorscheme::Plum, Mode::Dark) => "#aa99d1",

            (Colorscheme::Lavender, Mode::Light) => "#cca3d6",
            (Colorscheme::Lavender, Mode::Dark) => "#cca3d6",

            (Colorscheme::Lilac, Mode::Light) => "#dbb4d3",
            (Colorscheme::Lilac, Mode::Dark) => "#dbb4d3",

            (Colorscheme::Pink, Mode::Light) => "#edb7ca",
            (Colorscheme::Pink, Mode::Dark) => "#edb7ca",
        }
    }
}

pub trait CanonicalHash {
    fn write_canonical(&self, hasher: &mut blake3::Hasher);
}
impl CanonicalHash for u64 {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        h.update(&self.to_le_bytes());
    }
}

impl CanonicalHash for i64 {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        h.update(&self.to_le_bytes());
    }
}

impl CanonicalHash for str {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        h.update(&(self.len() as u64).to_le_bytes());
        h.update(self.as_bytes());
    }
}

impl CanonicalHash for String {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.as_str().write_canonical(h);
    }
}

impl CanonicalHash for NaiveDate {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.to_string().write_canonical(h);
    }
}

impl CanonicalHash for DateTime<Utc> {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.to_string().write_canonical(h);
    }
}

impl CanonicalHash for Color {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.as_str().write_canonical(h);
    }
}

impl CanonicalHash for TagId {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.id.write_canonical(h);
        // Name is intentionally ignored, since the value could be stale later
        // (This is by design, see Eri design docs).
    }
}

impl CanonicalHash for ProjectId {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.id.write_canonical(h);
        // Name is intentionally ignored, since the value could be stale later
        // (This is by design, see Eri design docs).
    }
}

impl<T: CanonicalHash> CanonicalHash for Option<T> {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        match self {
            None => {
                h.update(&[0]);
            }
            Some(v) => {
                h.update(&[1]);
                v.write_canonical(h);
            }
        }
    }
}

impl<T: CanonicalHash> CanonicalHash for [T] {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        h.update(&(self.len() as u64).to_le_bytes());
        for item in self {
            item.write_canonical(h);
        }
    }
}

impl<T: CanonicalHash> CanonicalHash for Vec<T> {
    fn write_canonical(&self, h: &mut blake3::Hasher) {
        self.as_slice().write_canonical(h);
    }
}
