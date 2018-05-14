use git::commit::Commit;
use git::branch::Branch;
use git::contributor::Contributor;

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
    settings: &'a Settings,
    commits: Vec<Commit<'a>>,
    branches: Vec<Branch<'a>>,
    contributors: Vec<Contributor<'a>>
}
impl <'a>Repo<'a> {
    pub fn new(name: &'a str, settings: &'a Settings) -> Repo<'a> {
        Repo {
            name: name,
            settings: settings,
            commits: vec![],
            branches: vec![],
            contributors: vec![]
        }
    }
}