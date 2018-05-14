use git::commit::{ Commit };

pub struct Settings {

}
impl Settings {
    pub fn new() -> Settings {
        Settings {

        }
    }
}

pub struct Repo<'a> {
    name: &'a str,
    settings: Settings,
    commits: Vec<Commit<'a>>
}
impl <'a>Repo<'a> {
    pub fn new(name: &'a str) -> Repo<'a> {
        Repo {
            name: name,
            settings: Settings::new(),
            commits: vec![]
        }
    }
}