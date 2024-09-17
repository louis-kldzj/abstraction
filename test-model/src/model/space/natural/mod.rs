use crate::model::{display::DisplayInfo, Identifier};
use abstraction::Abstract;
use asteroid_belt::AsteroidBelt;
use planet::Planet;

pub mod asteroid_belt;
pub mod planet;

#[derive(Abstract)]
#[abstract_traits(Identifier, DisplayInfo)]
pub enum NaturalSpaceEntity {
    Planet(Planet),
    AsteroidBelt(AsteroidBelt),
}
