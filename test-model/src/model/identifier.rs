pub trait Identifier {
    fn get_id(&self) -> &str {
        &self.identifier_instance().get_id()
    }

    fn identifier_instance(&self) -> &dyn Identifier;
}

pub struct Identity {
    id: String,
}

impl Identifier for Identity {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn identifier_instance(&self) -> &dyn Identifier {
        self
    }
}
