pub struct Contributor <'a> {
    name: &'a str
}

impl <'a> Contributor <'a> {
    pub fn new(name: &'a str) -> Contributor<'a> {
        Contributor {
            name: name
        }
    }
}
