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
 
    println!("\nEntrez un préfixe à rechercher :");
    let mut prefixe = String::new();
    std::io::stdin().read_line(&mut prefixe)?;
    let prefixe = prefixe.trim();
 
    let resultats = trie.rechercher(prefixe);
    if resultats.is_empty() {
        println!("Aucun contact trouvé pour le préfixe '{}'", prefixe);
    } else {
        println!("Contacts trouvés pour '{}' :", prefixe);
        for nom in &resultats {
            println!("  -> {}", nom);
        }
    }
 
    println!("Trie construit avec {} contacts !", contacts.len());
    println!("Fichier PlantUML généré dans graph/contacts.puml !");
 
    Ok(())
}