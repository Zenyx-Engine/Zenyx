pub(crate) mod hash_db;
use hash_db::*;

pub(crate) mod test;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MyType {
        field: String,
    }

    // For custom types, implement the TypeDowncaster trait
    #[derive(Debug)]
    struct MyCustomType(String);

    #[test]
    fn it_works() {
        let mut ECSDB = hash_db::HashDb::new();
        ECSDB.insert("key1", "value1".to_string());
        ECSDB.insert("key2", MyCustomType("custom".to_string()));

        let value1 = get_typed!(&ECSDB, "key1", String);
        let value2 = get_typed!(&ECSDB, "key2", MyCustomType);
        
        println!("ecsdb string part: {:?}", value1);
        println!("ecsdb custom part: {:?}", value2);
    }
}