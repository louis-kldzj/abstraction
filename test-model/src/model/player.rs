use abstraction::Concrete;

use super::{
    display::{DisplayData, DisplayInfo},
    identifier::{Identifier, Identity},
    GameObject,
};

#[derive(Concrete)]
#[concrete_traits(Identifier, DisplayInfo, GameObject)]
pub struct Player {
    #[data_field(Identifier)]
    identity: Identity,
    #[data_field(DisplayInfo)]
    display_data: DisplayData,
}
