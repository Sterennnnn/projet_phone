use phone_trie::{lire_contacts, generer_plantuml};
use phone_trie::trie::Trie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contacts = lire_contacts("data/contacts.json")?;

    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert(&contact.phone, &contact.name);
    }

    let puml = trie.to_plantuml();
    generer_plantuml("graph/contacts.puml", &puml)?;
    println!("Trie construit avec {} contacts !", contacts.len());
    println!("Fichier PlantUML généré dans graph/contacts.puml !");

    Ok(())
}