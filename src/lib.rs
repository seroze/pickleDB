pub mod error;
pub mod datatype;
pub mod db; 

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get(){
     ///Get `value` with the specified `key`, return a empty optional if `key` is not contained in the database

        use crate::{datatype::DataType, db::PickleDB};

        let mut database = PickleDB::default();
        let data = String::from("data");
        let key = String::from("example");
        database.set(key.clone(), data.clone());
        let result = database.get(&key).unwrap();
        assert_eq!(*result, DataType::STRING(data));

    }


}
