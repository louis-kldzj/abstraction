use abstraction::Abstract;
use display::DisplayInfo;
use identifier::Identifier;
use player::Player;
use space::SpaceEntity;

mod display;
mod identifier;
mod player;
mod space;

pub trait GameObject: DisplayInfo + Identifier {
    fn gameobject_instance(&self) -> &dyn GameObject;

    fn gameobject_instance_mut(&mut self) -> &mut dyn GameObject;
}

#[derive(Abstract)]
#[abstract_traits(Identifier, DisplayInfo)]
enum Entity {
    Space(SpaceEntity),
    Player(Player),
}
