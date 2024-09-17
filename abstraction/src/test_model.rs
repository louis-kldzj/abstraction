use crate::abstraction;

#[test]
fn works() {}

pub trait GameObject: DisplayInfo + Identifier {
    fn test(&self) -> &str {
        "SUCCESS"
    }
}

pub trait DisplayInfo {
    fn get_name(&self) -> &str {
        &self.instance().get_name()
    }

    fn get_icon(&self) -> &char {
        &self.instance().get_icon()
    }

    fn instance(&self) -> &dyn DisplayInfo;
}

pub trait Identifier {
    fn get_id(&self) -> &str {
        &self.instance().get_id()
    }

    fn instance(&self) -> &dyn Identifier;
}

struct Identity {
    id: String,
}

impl Identifier for Identity {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn instance(&self) -> &dyn Identifier {
        self
    }
}

struct DisplayData {
    name: String,
    icon: char,
}

impl DisplayInfo for DisplayData {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_icon(&self) -> &char {
        &self.icon
    }

    fn instance(&self) -> &dyn DisplayInfo {
        self
    }
}

enum Entity {
    Space(SpaceEntity),
    Player(Player),
}

struct Player {
    identity: Identity,
    display_data: DisplayData,
}

impl Identifier for Player {
    fn instance(&self) -> &dyn Identifier {
        &self.identity
    }
}

impl DisplayInfo for Player {
    fn instance(&self) -> &dyn DisplayInfo {
        &self.display_data
    }
}

pub enum SpaceEntity {
    Natrual(),
    Construct(ConstructSpaceEntity),
}

#[abstraction(Identifier)]
pub enum NaturalSpaceEntity {
    Planet(Planet),
    AsteroidBelt(AsteroidBelt),
}

struct Planet {
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

struct AsteroidBelt {
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

enum ConstructSpaceEntity {
    Structure(Structure),
}

struct Structure {
    id: String,
}

impl Identifier for Structure {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn instance(&self) -> &dyn Identifier {
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

    fn instance(&self) -> &dyn DisplayInfo {
        self
    }
}
