pub trait Identifier {
    fn get_id(&self) -> &str {
        &self.instance().get_id()
    }

    fn instance(&self) -> &dyn Identifier;
}

pub struct Identity {
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
