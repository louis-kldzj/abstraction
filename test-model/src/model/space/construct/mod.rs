use abstraction::Abstract;

use crate::model::{display::DisplayInfo, identifier::Identifier};

#[derive(Abstract)]
#[abstract_traits(Identifier, DisplayInfo)]
pub enum ConstructSpaceEntity {
    Structure(Structure),
}

pub struct Structure {
    id: String,
}

impl Identifier for Structure {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn identifier_instance(&self) -> &dyn Identifier {
        self
    }
}

impl DisplayInfo for Structure {
    fn get_name(&self) -> &str {
        "AG"
    }

    fn get_icon(&self) -> &char {
        &'&'
    }

    fn displayinfo_instance(&self) -> &dyn DisplayInfo {
        self
    }
}
