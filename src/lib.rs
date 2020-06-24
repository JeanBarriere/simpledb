use std::collections::HashMap;

pub trait SimpleDB {
    fn set(&mut self, key: String, value: u32);
    fn get(&mut self, key: String) -> Option<&u32>;
    fn unset(&mut self, key: String);
    fn begin_transaction(&mut self);
    fn rollback(&mut self) -> Result<(), String>;
    fn commit(&mut self) -> Result<(), String>;
}

pub type Store = HashMap<String, u32>;


// With optimized DB, loop over vec from the rev() and find get, set by pushing at the end
// rollback by splitting store at last transaction
// commit by clearing transactions
// pub struct OptimizedDB {
//     store: Vec<(String, u32)>,
//     transactions: Vec<u8>
// }

pub struct InMemoryDB {
    store: Store,
    history: Vec<Store>
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB { store: HashMap::new(), history: vec![] }
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {
        self.store.insert(key, value);
    }

    fn get(&mut self, key: String) -> Option<&u32> {
        self.store.get(&key)
    }

    fn unset(&mut self, key: String) {
        self.store.remove(&key);
    }

    fn begin_transaction(&mut self) {
        self.history.push(self.store.clone());
    }

    fn rollback(&mut self) -> Result<(), String> {
        match self.history.split_last() {
            Some((last, history)) => {
                self.store = last.clone();
                self.history = history.to_vec();
                Ok(())
            },
            None => Err(String::from("No transactions in progress"))
        }
    }

    fn commit(&mut self) -> Result<(), String> {
        if self.history.is_empty() {
            Err(String::from("No transactions in progress"))
        } else {
            self.history = vec![];
            Ok(())
        }
    }
}
