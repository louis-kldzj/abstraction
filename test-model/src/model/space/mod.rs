use abstraction::Abstract;
use construct::ConstructSpaceEntity;
use natural::NaturalSpaceEntity;

use super::{display::DisplayInfo, identifier::Identifier};

pub mod construct;
pub mod natural;

#[derive(Abstract)]
#[abstract_traits(Identifier, DisplayInfo)]
pub enum SpaceEntity {
    Natrual(NaturalSpaceEntity),
    Construct(ConstructSpaceEntity),
}
