use std::fs;
use serde::Deserialize;

mod trie;
use trie::Trie;

#[derive(Deserialize, Debug)]
struct Contact {
    name: String,
    phone: String,
}

fn main() {
    let data = fs::read_to_string("data/contacts.json")
        .expect("Impossible de lire le fichier JSON");

    let contacts: Vec<Contact> = serde_json::from_str(&data)
        .expect("Erreur de parsing JSON");

    let mut mon_trie = Trie::new();

    for contact in &contacts {
        mon_trie.insert(&contact.phone, &contact.name);
        println!("Inséré : {} -> {}", contact.name, contact.phone);
    }

    println!("\nTrie construit avec {} contacts !", contacts.len());
}