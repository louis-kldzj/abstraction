pub trait DisplayInfo {
    fn get_name(&self) -> &str {
        &self.instance().get_name()
    }

    fn get_icon(&self) -> &char {
        &self.instance().get_icon()
    }

    fn instance(&self) -> &dyn DisplayInfo;
}

pub struct DisplayData {
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
