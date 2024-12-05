use std::collections::HashMap;
use std::any::Any;

pub struct HashDb {
    db: HashMap<String, Box<dyn Any>>,
}



impl HashDb {
    pub fn new() -> Self {
        HashDb {
            db: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, key: &str, value: T) {
        self.db.insert(key.to_string(), Box::new(value));
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        self.db.get(key).and_then(|value| value.downcast_ref::<T>())
    }

    // Get the type name of a stored value
    pub fn get_type(&self, key: &str) -> Option<&'static str> {
        self.db.get(key).map(|value| std::any::type_name_of_val(&**value))
    }

    // Check if a value is of a specific type
    pub fn is_type<T: Any>(&self, key: &str) -> bool {
        self.db.get(key).map_or(false, |value| value.is::<T>())
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}

#[macro_export]
macro_rules! get_typed {
    ($db:expr, $key:expr, $type:ty) => {
        $db.get::<$type>($key)
    };
}

