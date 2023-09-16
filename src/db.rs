// use crate::*;
use crate::error::Error;
use crate::error::Result; 
use crate::datatype::DataType;


use std::{
    collections::{hash_map::IntoIter, HashMap, VecDeque},
    fs::File,
    io::{Read, Write},
    iter::IntoIterator,
    mem::size_of,
    path::Path,
};
use yazi::{compress, decompress, CompressionLevel, Format};

macro_rules! from_be_bytes {
    ($type_name:ty, $data_buffer:ident) => {
        <$type_name>::from_be_bytes(
            $data_buffer
                .drain(0..size_of::<$type_name>())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| Error::DecodeError)?,
        )
    };
}

///Object to represent the in memory database
#[derive(Debug, PartialEq, Default)]
pub struct PickleDB {
    inner: HashMap<String, DataType>,
}

impl PickleDB {
    fn save_file(&self, filename: impl AsRef<Path>, data: &[u8]) -> Result<()> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");
        File::create(filename)?.write_all(data)?;
        Ok(())
    }

    fn read_file(filename: impl AsRef<Path>) -> Result<Vec<u8>> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");
        let mut buffer: Vec<u8> = Vec::new();
        File::open(filename)?.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded_data = Vec::<u8>::new();
        for (key, value) in self.inner.iter() {
            encoded_data.extend(key.len().to_be_bytes());
            encoded_data.extend(key.as_bytes());
            encoded_data.push(value.id());
            match value {
                DataType::BOOL(value) => encoded_data.push(*value as u8),
                DataType::INT8(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT16(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT32(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT64(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::FLOAT32(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::FLOAT64(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::STRING(value) => {
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value.as_bytes());
                }
                DataType::BYTES(value) => {
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value);
                }
            }
        }
        encoded_data
    }

    fn decode(mut encoded_data: VecDeque<u8>) -> Result<HashMap<String, DataType>> {
        let mut db_hashmap = HashMap::new();
        while !encoded_data.is_empty() {
            let key_size = from_be_bytes!(usize, encoded_data);
            let key = String::from_utf8(encoded_data.drain(0..key_size).collect())
                .map_err(|_| Error::DecodeError)?;
            match encoded_data.pop_front().ok_or(Error::DecodeError)? {
                0 => {
                    let data = encoded_data.pop_front().ok_or(Error::DecodeError)? != 0;
                    db_hashmap.insert(key.clone(), DataType::BOOL(data));
                }
                1 => {
                    let data = from_be_bytes!(i8, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT8(data));
                }
                2 => {
                    let data = from_be_bytes!(i16, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT16(data));
                }
                3 => {
                    let data = from_be_bytes!(i32, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT32(data));
                }
                4 => {
                    let data = from_be_bytes!(i64, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT64(data));
                }
                5 => {
                    let data = from_be_bytes!(f32, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::FLOAT32(data));
                }
                6 => {
                    let data = from_be_bytes!(f64, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::FLOAT64(data));
                }
                7 => {
                    let data_size = from_be_bytes!(usize, encoded_data);
                    let data = String::from_utf8(encoded_data.drain(0..data_size).collect())
                        .map_err(|_| Error::DecodeError)?;
                    db_hashmap.insert(key.clone(), DataType::STRING(data));
                }
                8 => {
                    let size = from_be_bytes!(usize, encoded_data);
                    let data = encoded_data.drain(0..size).collect::<Vec<u8>>();
                    db_hashmap.insert(key.clone(), DataType::BYTES(data));
                }
                _ => {
                    return Err(Error::DecodeError);
                }
            }
        }
        Ok(db_hashmap)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let encoded_data = Self::read_file(path)?;
        let (encoded_data, _) = decompress(&encoded_data, Format::Zlib)?;
        let data = Self::decode(encoded_data.into())?;
        Ok(Self { inner: data })
    }

 
    pub fn backup(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)?;
        self.save_file(path, &data)
    }
   
    pub fn load_from_stream(stream: &mut impl Read) -> Result<Self> {
        let mut encoded_data = Vec::new();
        stream.read_to_end(&mut encoded_data)?;
        let (encoded_data, _) = decompress(&encoded_data, Format::Zlib)?;
        let data = Self::decode(encoded_data.into())?;
        Ok(Self { inner: data })
    }
  
    pub fn backup_to_stream(&self, stream: &mut impl Write) -> Result<()> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)?;
        stream.write_all(&data)?;
        Ok(())
    }
   

    #[inline]
    pub fn set(&mut self, key: impl ToString, value: impl Into<DataType>) -> Option<DataType> {
        self.inner.insert(key.to_string(), value.into())
    }
   

    #[inline]
    pub fn get(&self, key: &impl ToString) -> Option<&DataType> {
        self.inner.get(&key.to_string())
    }
   
   
    pub fn contains_key(&self, key: &impl ToString) -> bool {
        self.inner.contains_key(&key.to_string())
    }
    
    
    pub fn remove(&mut self, key: &impl ToString) -> Option<DataType> {
        self.inner.remove(&key.to_string())
    }
   
   
    #[inline]
    pub fn extract<'c,T: TryFrom<&'c DataType, Error = Error>>(&'c self, key: &impl ToString) -> Option<Result<T>>{
        self.get(key).map(T::try_from)
    }
}

impl IntoIterator for PickleDB {
    type Item = (String, DataType);

    type IntoIter = IntoIter<String, DataType>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}