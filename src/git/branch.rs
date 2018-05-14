pub struct Branch <'a> {
    name: &'a str
}

impl <'a> Branch <'a> {
    pub fn new(name: &'a str) -> Branch<'a> {
        Branch {
            name: name
        }
    }
}
