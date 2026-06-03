use std::collections::HashMap;

pub struct Interner {
    dict: HashMap<String, usize>,
    words: Vec<String>,
}

impl Interner {
    // methods
    pub fn new() -> Self {
        Interner {
            dict: HashMap::new(),
            words: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(w) = self.dict.get(s) {
            *w
        } else {
            let new_id: usize = self.words.len();
            self.words.push(s.to_string());
            self.dict.insert(s.to_string(), new_id);
            new_id
        }
    }

    pub fn lookup(&self, id: usize) -> &str {
        &self.words[id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intern_same_string_returns_same_id() {
        let mut interner = Interner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("hello");
        assert_eq!(id1, id2);
    }

    #[test]
    fn intern_different_strings_returns_different_ids() {
        let mut interner = Interner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");
        assert_ne!(id1, id2);
    }

    #[test]
    fn lookup_returns_correct_string() {
        let mut interner = Interner::new();
        let id = interner.intern("hello");
        assert_eq!(interner.lookup(id), "hello");
    }

    #[test]
    fn ids_are_sequential_0_to_99() {
        let mut interner = Interner::new();
        for i in 0..100 {
            let id = interner.intern(&format!("string_{}", i));
            assert_eq!(id, i);
        }
    }
}
