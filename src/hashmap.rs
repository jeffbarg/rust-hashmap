use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Index;

pub struct HashMap<K, V> {
    elements: Vec<(K, Vec<V>)>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        let default_capacity: usize = 20;
        Self {
            elements: Vec::with_capacity(default_capacity),
        }
    }

    pub fn insert(&mut self, key: V, value: V) {
        unimplemented!()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unimplemented!()
    }
}

impl<K, V> Index<&K> for HashMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_retrieval() {
        // Type inference lets us omit an explicit type signature (which
        // would be `HashMap<String, String>` in this example).
        let mut book_reviews: HashMap<String, String> = HashMap::new();

        // Review some books.
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );

        // Look up the value for a key (will panic if the key is not found).
        assert_eq!(book_reviews.contains_key("Pride and Prejudice"), true);
    }
}
