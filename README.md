# Rust: Hash Table
I've implemented a hash table (also called a hash map) as practice to gain insights on how the `std::collections::HashMap` might work under the hood.

## Collision Resolution
I'm going to handle collisions using [linear probing](https://en.wikipedia.org/wiki/Linear_probing) a form of open addressing where we search (probe) adjacent tables until we find one that's free (when inserting) or contains the key. It's worth noting that linear probing degrades in performance considerably with a high enough load factor, so once we hit 0.75 (about 75% full) we resize the vector.

## API
1. `new()`: Creates a new hash table
2. `len()`: Returns the number of occupied entries
3. `insert(key: Key)`: Inserts an `Entry` into the hash table
4. `get(key: Key)`: Returns an `Option<Entry>` for that key
5. `get_mut(key: Key)`: Returns an `Option<&mut Entry>` mutable reference for that key
6. `remove()`: Removes an item from the hash table
