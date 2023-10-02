# Rust: Hash Table
I've implemented a hash table (also called a hash map) as practice to gain insights on how the `std::collections::HashMap` works under the hood.

## API
1. `new()`: Creates a new hash table
2. `len()`: Returns the number of occupied entries
3. `insert(key: Key)`: Inserts an `Entry` into the hash table
4. `get(key: Key)`: Returns an `Option<Entry>` for that key
5. `get_mut(key: Key)`: Returns an `Option<&mut Entry>` mutable reference for that key
6. `remove()`: Removes an item from the hash table
