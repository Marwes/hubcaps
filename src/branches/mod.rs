//! Gists interface
extern crate serde_json;

use self::super::{MediaType, Error, Github, Result};
use url::form_urlencoded;
use users::User;
use std::collections::HashMap;
use std::hash::Hash;

/// reference to gists associated with a github user
pub struct Branches<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Branches<'a> {
    pub fn new<U, R>(github: &'a Github, owner: U, repo: R) -> Self
        where U: Into<String>,
              R: Into<String>
    {
        Branches {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// list of teams for this org
    pub fn list(&self) -> Result<Vec<Branch>> {
        self.github.get_media::<Vec<Branch>>(&format!("/repos/{owner}/{repo}/branches",
                                                      owner = self.owner,
                                                      repo = self.repo),
                                             MediaType::Preview("loki"))
    }
}


// representations

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub protected: bool,
    pub protection_url: String, // pub commit: CommitRef
    pub protection: Protection,
}

#[derive(Debug, Deserialize)]
pub struct Protection {
    pub enabled: bool,
    pub required_status_checks: StatusChecks,
}

#[derive(Debug, Deserialize)]
pub struct StatusChecks {
    pub enforcement_level: String,
    pub contexts: Vec<String>,
}