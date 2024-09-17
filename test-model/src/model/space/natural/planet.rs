use abstraction::Concrete;

use crate::model::{
    display::{DisplayData, DisplayInfo},
    identifier::{Identifier, Identity},
};

#[derive(Concrete)]
#[concrete_traits(Identifier, DisplayInfo)]
pub struct Planet {
    #[data_field(Identifier)]
    identity: Identity,
    #[data_field(DisplayInfo)]
    display_data: DisplayData,
}
