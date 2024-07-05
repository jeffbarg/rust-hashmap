use std::borrow::Borrow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Index;

pub struct HashMap<K, V> {
    elements: Box<[Option<(K, V)>]>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        let default_capacity: usize = 20;

        // Build an empty elements vector
        let mut elements_vec = Vec::with_capacity(default_capacity);
        for _ in 0..default_capacity {
            elements_vec.push(Option::None);
        }

        Self {
            elements: elements_vec.into_boxed_slice(),
        }
    }

    // Private implementation of hashing a key
    fn hash_key(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        hash
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = self.hash_key(&key);

        // Get the relevant tuple from the hash
        let elements_len = self.elements.len();
        let hashed_index = (hash as usize) % elements_len;

        // Get the element at the hashed index, and see if there's a collision
        match self.elements[hashed_index].take() {
            None => {
                // Set the value at the index
                self.elements[hashed_index] = Some((key, value));

                // Return `None` since there was no existing value
                None
            }
            Some((existing_key, existing_value)) => {
                if key == existing_key {
                    // We need to replace the value that we've already swapped out above with `.take()`
                    self.elements[hashed_index] = Some((key, value));

                    // There's an existing value here that we are overriting and returning
                    Some(existing_value)
                } else {
                    panic!("There's a hash collision")
                }
            }
        }
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Hash + Eq,
    {
        let hash = self.hash_key(key);

        // Get the relevant tuple from the hash
        let elements_len = self.elements.len();
        let hashed_index = (hash as usize) % elements_len;

        // Get the element at the hashed index, and see if there's a collision
        self.elements[hashed_index]
            .as_ref()
            .map(|(existing_key, _)| existing_key == key)
            .unwrap_or(false)
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
        assert_eq!(
            book_reviews.contains_key(&"Pride and Prejudice".to_string()),
            true
        );
    }
}
