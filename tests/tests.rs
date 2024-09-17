use abstraction::{Abstract, Concrete};

pub struct Data {
    name: String,
}

pub trait Info {
    fn get_name(&self) -> &String {
        &self.data().name
    }

    fn data(&self) -> &Data;
}

#[derive(Abstract)]
#[abstract_trait(Info)]
#[data_struct(Data)]
pub enum TopLevel {
    AbstractA(A),
    AbstractB(B),
}

#[derive(Concrete)]
#[concrete_trait(Info)]
#[data_struct(Data)]
pub struct A {
    data: Data,
}

#[derive(Concrete)]
#[concrete_trait(Info)]
#[data_struct(Data)]
pub struct B {
    data: Data,
}

#[test]
fn concrete_works() {
    let a = A {
        data: Data {
            name: "Gabagool".to_string(),
        },
    };

    assert_eq!(a.get_name().as_str(), "Gabagool")
}
