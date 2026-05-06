#![forbid(unsafe_code)]

pub mod trie;

use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub name: String,
    pub phone: String,
}

pub fn lire_contacts(chemin: &str) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(chemin)?;
    let contacts = serde_json::from_str(&data)?;
    Ok(contacts)
}

pub fn generer_plantuml(chemin: &str, contenu: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(chemin, contenu)?;
    Ok(())
}