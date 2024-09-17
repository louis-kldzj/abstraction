use crate::model::{
    display::{DisplayData, DisplayInfo},
    identifier::{Identifier, Identity},
};

pub struct Planet {
    identity: Identity,
    display_data: DisplayData,
}

impl Identifier for Planet {
    fn instance(&self) -> &dyn Identifier {
        &self.identity
    }
}

impl DisplayInfo for Planet {
    fn instance(&self) -> &dyn DisplayInfo {
        &self.display_data
    }
}
