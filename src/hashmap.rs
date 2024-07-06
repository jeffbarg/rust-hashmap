use std::hash::{DefaultHasher, Hash, Hasher};

const DEFAULT_CAPACITY: usize = 20;
const LOAD_FACTOR: f64 = 0.7;

// Private implementation of hashing a key
fn hash_key<K>(key: &K) -> u64
where
    K: Hash,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash = hasher.finish();

    hash
}

pub struct HashMap<K, V> {
    elements: Box<[Option<(K, V)>]>,
    size: usize,
    capacity: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        // Build an empty elements vector
        let mut elements_vec = Vec::with_capacity(DEFAULT_CAPACITY);
        for _ in 0..DEFAULT_CAPACITY {
            elements_vec.push(Option::None);
        }

        let elements = elements_vec.into_boxed_slice();
        Self {
            elements,
            size: 0,
            capacity: DEFAULT_CAPACITY,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check if we need to resize
        self.resize_if_needed();

        let hash = hash_key(&key);

        // Get the relevant tuple from the hash
        let elements_len = self.capacity;
        let mut hashed_index = (hash as usize) % elements_len;
        let mut num_iterations = 0;

        while num_iterations < self.capacity {
            // Get the element at the hashed index, and see if there's a collision
            match self.elements[hashed_index].take() {
                None => {
                    // Set the value at the index
                    self.elements[hashed_index] = Some((key, value));
                    self.size += 1;

                    // Return `None` since there was no existing value
                    return None;
                }
                Some((existing_key, existing_value)) => {
                    if key == existing_key {
                        // We need to replace the value that we've already swapped out above with `.take()`
                        self.elements[hashed_index] = Some((key, value));

                        // There's an existing value here that we are overriting and returning
                        return Some(existing_value);
                    } else {
                        num_iterations += 1;
                        hashed_index += 1;
                        hashed_index %= elements_len;
                        continue;
                    }
                }
            };
        }

        // If iterated through the whole list, and the element was not able to be inserted, something went seriously wrong
        // `resize_if_needed` which is called at the outset of this function, doubles the size of the array if it is at 70% (the LOAD_FACTOR) fullness.
        panic!("Couldn't insert element after iterating through all possible slots")
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Hash + Eq,
    {
        return self.get(key).map(|_| true).unwrap_or(false);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = hash_key(key);

        // Get the relevant tuple from the hash
        let elements_len = self.capacity;

        let mut hashed_index = (hash as usize) % elements_len;
        let mut num_iterations = 0;

        // Get the element at the hashed index, and see if there's a collision
        while num_iterations < self.capacity {
            match self.elements[hashed_index].as_ref() {
                Some((existing_key, value)) => {
                    if *key == *existing_key {
                        return Some(value);
                    } else {
                        println!(
                            "Hash collision {} {} {}",
                            num_iterations, hashed_index, hash
                        );
                        num_iterations += 1;
                        hashed_index += 1;
                        hashed_index %= elements_len;
                        continue;
                    }
                }
                None => {
                    return None;
                }
            }
        }

        // If iterated through the whole list, and the element was either not found or an empty slot was not found, something went seriously wrong
        // `resize_if_needed` which is called at the outset of this function, doubles the size of the array if it is at 70% (the LOAD_FACTOR) fullness. An empty slot *should* exist, so we should never get here.
        panic!("Couldn't find element or empty slot after iterating through all possible slots")
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash = hash_key(key);

        // Get the relevant tuple from the hash
        let elements_len = self.capacity;
        let mut hashed_index = (hash as usize) % elements_len;
        let mut num_iterations = 0;

        // Get the element at the hashed index, and see if there's a collision
        while num_iterations < self.capacity {
            let cur_el = self.elements[hashed_index].as_ref();
            match cur_el {
                Some((existing_key, _)) => {
                    if *existing_key == *key {
                        // This is the right index, take the element out of the option and return
                        return self.elements[hashed_index].take().map(|(_, v)| v);
                    } else {
                        num_iterations += 1;
                        hashed_index += 1;
                        hashed_index %= elements_len;
                        continue;
                    }
                }
                None => {
                    // The element was not in the map
                    return None;
                }
            }
        }

        // If iterated through the whole list, and the element was either not found or an empty slot was not found, something went seriously wrong
        // `resize_if_needed` which is called at the outset of this function, doubles the size of the array if it is at 70% (the LOAD_FACTOR) fullness. An empty slot *should* exist, so we should never get here.
        panic!("Couldn't find element or empty slot after iterating through all possible slots")
    }

    fn resize_if_needed(&mut self) {
        let new_size = self.size + 1;
        if (new_size as f64) > (self.capacity as f64) * LOAD_FACTOR {
            println!("Resizing {}", new_size);
            self.resize_underlying_table()
        }
    }
    fn resize_underlying_table(&mut self) {
        // First allocate new space
        let new_capacity = self.capacity * 2;

        let mut elements_vec = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            elements_vec.push(Option::None);
        }

        for item in self.elements.iter_mut() {
            item.take().map(|(key, value)| {
                let hash = hash_key(&key);
                let mut hashed_index = (hash as usize) % new_capacity;
                while elements_vec[hashed_index].is_some() {
                    hashed_index += 1;
                    hashed_index %= new_capacity;
                }
                elements_vec[hashed_index] = Some((key, value));
            });
        }

        self.elements = elements_vec.into_boxed_slice();
        self.capacity = new_capacity;
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

        // Look up the value for a key
        assert_eq!(
            book_reviews.get(&"Pride and Prejudice".to_string()),
            Some(&"Very enjoyable.".to_string())
        );
        assert_ne!(
            book_reviews.get(&"Pride and Prejudice".to_string()),
            Some(&"A different string review.".to_string())
        );
    }

    #[test]
    fn test_resizing_by_adding_many_items() {
        let mut test_map: HashMap<i64, i64> = HashMap::new();

        for i in 0..1000 {
            test_map.insert(3 * i, i * 8 + 5);
        }

        assert_eq!(test_map.size, 1000);
        assert_eq!(test_map.capacity, 2560);
        assert!(test_map.contains_key(&3));
        assert!(test_map.contains_key(&63));
        assert!(test_map.contains_key(&33));
        assert!(test_map.contains_key(&300));
    }

    #[test]
    fn test_deletes() {
        let mut test_map: HashMap<i64, i64> = HashMap::new();

        for i in 0..10 {
            test_map.insert(3 * i, i * 8 + 5);
        }

        assert_eq!(test_map.get(&9), Some(&29));
        assert_eq!(test_map.remove(&9), Some(29));
        assert_eq!(test_map.get(&9), None);
    }
}
