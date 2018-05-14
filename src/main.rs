extern crate uuid;

mod git;

use git::repo::{ Repo };

fn main() {

    let repo = git::repo::Repo::new("test");

}
