use crate::model::{
    display::{DisplayData, DisplayInfo},
    identifier::{Identifier, Identity},
};

pub struct AsteroidBelt {
    identity: Identity,
    display_data: DisplayData,
}

impl Identifier for AsteroidBelt {
    fn instance(&self) -> &dyn Identifier {
        &self.identity
    }
}

impl DisplayInfo for AsteroidBelt {
    fn instance(&self) -> &dyn DisplayInfo {
        &self.display_data
    }
}
