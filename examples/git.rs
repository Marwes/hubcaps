extern crate env_logger;
extern crate hyper;
extern crate hubcaps;
extern crate tokio_core;

use std::env;

use tokio_core::reactor::Core;

use hubcaps::{Credentials, Github};

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            for file in core.run(github.repo("softprops", "hubcaps").git().tree(
                "master",
                true,
            )).unwrap()
                .tree
                .iter()
                .find(|file| file.path == "README.md")
            {
                let blob = core.run(github.repo("softprops", "hubcaps").git().blob(
                    file.sha.clone(),
                )).unwrap();
                println!("readme {:#?}", blob);
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
