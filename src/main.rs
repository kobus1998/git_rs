extern crate uuid;

mod git;

use git::repo::{ Repo, Settings };

fn main() {

    let settings = Settings::new();
    let repo = git::repo::Repo::new("test", &settings);

}
