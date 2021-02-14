use crate::common::logging::{self, Logger};
use crate::store::person::Person;
use crate::store::Store;

#[derive(Clone)]
pub(crate) struct Backend {
    store: Store,
    logger: Logger,
}

impl Backend {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            logger: logging::get_logger("backend"),
        }
    }

    pub fn create_person(&self) -> Person {
        self.store.create_person()
    }

    pub fn read_person(&self) -> Person {
        self.store.create_person()
    }
}
