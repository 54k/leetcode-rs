// https://leetcode.com/problems/implement-trie-prefix-tree/
#[derive(Debug, Clone)]
struct TrieNode {
    links: Vec<Option<TrieNode>>,
    is_end: bool,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            links: vec![None; 26],
            is_end: false,
        }
    }
    fn contains_key(&self, key: char) -> bool {
        self.links[key as usize - 'a' as usize].is_some()
    }
    fn get(&self, key: char) -> Option<&TrieNode> {
        self.links[key as usize - 'a' as usize].as_ref()
    }
    fn get_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.links[key as usize - 'a' as usize].as_mut()
    }
    fn put(&mut self, key: char, node: TrieNode) {
        self.links[key as usize - 'a' as usize] = Some(node);
    }
}
struct Trie {
    root: TrieNode,
}
impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
    fn insert(&mut self, word: String) {
        let mut root = &mut self.root;
        for ch in word.chars() {
            if !root.contains_key(ch) {
                root.put(ch, TrieNode::new());
            }
            root = root.get_mut(ch).unwrap();
        }
        root.is_end = true;
    }
    fn search(&self, word: String) -> bool {
        let mut root = &self.root;
        for ch in word.chars() {
            if !root.contains_key(ch) {
                return false;
            }
            root = root.get(ch).unwrap();
        }
        root.is_end
    }
    fn starts_with(&self, prefix: String) -> bool {
        let mut root = &self.root;
        for ch in prefix.chars() {
            if !root.contains_key(ch) {
                return false;
            }
            root = root.get(ch).unwrap();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test352() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        println!("{}", trie.search("apple".to_string())); // true
        println!("{}", trie.search("app".to_string())); // false
        println!("{}", trie.starts_with("app".to_string())); // true
        trie.insert("app".to_string());
        println!("{}", trie.search("app".to_string())); // true
    }
}
