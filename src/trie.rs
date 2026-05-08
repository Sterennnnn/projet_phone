pub struct TrieNode {
    pub children: [Option<Box<TrieNode>>; 10],
    pub contact_name: Option<String>,
}
 
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: std::array::from_fn(|_| None),
            contact_name: None,
        }
    }
}
 
pub struct Trie {
    pub root: TrieNode,
}
 
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
 
    pub fn insert(&mut self, phone: &str, name: &str) {
        let mut node = &mut self.root;
        for ch in phone.chars() {
            let index = ch.to_digit(10).expect("Caractère invalide") as usize;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }
        node.contact_name = Some(name.to_string());
    }
 
    pub fn to_plantuml(&self) -> String {
        let mut output = String::from("@startmindmap\n* root\n");
        self.node_to_plantuml(&self.root, 1, &mut output);
        output.push_str("@endmindmap\n");
        output
    }
 
    fn node_to_plantuml(&self, node: &TrieNode, depth: usize, output: &mut String) {
        for (i, child) in node.children.iter().enumerate() {
            if let Some(child_node) = child {
                let stars = "*".repeat(depth + 1);
                if let Some(name) = &child_node.contact_name {
                    output.push_str(&format!("{} {} ({})\n", stars, i, name));
                } else {
                    output.push_str(&format!("{} {}\n", stars, i));
                }
                self.node_to_plantuml(child_node, depth + 1, output);
            }
        }
    }
 
    pub fn rechercher(&self, prefixe: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefixe.chars() {
            let index = ch.to_digit(10).expect("Caractère invalide") as usize;
            match &node.children[index] {
                None => return vec![],
                Some(enfant) => node = enfant,
            }
        }
        let mut resultats = vec![];
        self.collecter_contacts(node, &mut resultats);
        resultats
    }
 
    fn collecter_contacts(&self, node: &TrieNode, resultats: &mut Vec<String>) {
        if let Some(nom) = &node.contact_name {
            resultats.push(nom.clone());
        }
        for enfant in &node.children {
            if let Some(enfant_node) = enfant {
                self.collecter_contacts(enfant_node, resultats);
            }
        }
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_insertion_simple() {
        let mut trie = Trie::new();
        trie.insert("0612345678", "Alice");
        assert!(trie.root.children[0].is_some());
    }
 
    #[test]
    fn test_contact_en_fin_de_numero() {
        let mut trie = Trie::new();
        trie.insert("06", "Alice");
        let noeud_0 = trie.root.children[0].as_ref().unwrap();
        let noeud_6 = noeud_0.children[6].as_ref().unwrap();
        assert_eq!(noeud_6.contact_name, Some("Alice".to_string()));
    }
 
    #[test]
    fn test_plusieurs_contacts() {
        let mut trie = Trie::new();
        trie.insert("06", "Alice");
        trie.insert("07", "Bob");
        let noeud_0 = trie.root.children[0].as_ref().unwrap();
        assert!(noeud_0.children[6].is_some());
        assert!(noeud_0.children[7].is_some());
    }
 
    #[test]
    fn test_plantuml_contient_root() {
        let trie = Trie::new();
        let puml = trie.to_plantuml();
        assert!(puml.contains("@startmindmap"));
        assert!(puml.contains("root"));
        assert!(puml.contains("@endmindmap"));
    }
 
    #[test]
    fn test_trie_vide() {
        let trie = Trie::new();
        for enfant in &trie.root.children {
            assert!(enfant.is_none());
        }
    }
}