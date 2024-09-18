use super::{
    display::{DisplayData, DisplayInfo},
    identifier::{Identifier, Identity},
};

pub struct Player {
    identity: Identity,
    display_data: DisplayData,
}

impl Identifier for Player {
    fn identifier_instance(&self) -> &dyn Identifier {
        &self.identity
    }
}

impl DisplayInfo for Player {
    fn displayinfo_instance(&self) -> &dyn DisplayInfo {
        &self.display_data
    }
}
