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
}