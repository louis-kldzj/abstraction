pub trait Identifier {
    fn get_id(&self) -> &str {
        &self.identifier_instance().get_id()
    }

    fn identifier_instance(&self) -> &dyn Identifier;

    fn identifier_instance_mut(&mut self) -> &mut dyn Identifier;
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

    fn identifier_instance_mut(&mut self) -> &mut dyn Identifier {
        self
    }
}
