use uuid::{ Uuid, UuidVersion };

pub struct Commit <'a> {
    id: Uuid,
    desc: &'a str
}

impl <'a> Commit <'a> {
    pub fn new(desc: &'a str) -> Commit<'a> {
        let id = Uuid::new(UuidVersion::Sha1).unwrap();

        Commit {
            id: id,
            desc: desc
        }
    }
}
