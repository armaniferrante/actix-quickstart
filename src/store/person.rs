use crate::store::Store;
use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    job: String,
    place: String,
}

impl Store {
    pub fn create_person(&self) -> Person {
        Person {
            job: "eng".to_string(),
            place: "sf".to_string(),
        }
    }

    pub fn read_person(&self) -> Person {
        Person {
            job: "eng".to_string(),
            place: "sf".to_string(),
        }
    }
}
