# pickleDB

This is for educational & learning purpose, the original code is present at https://github.com/ninomerlino/SmollDB 

Architecture : 

- pickleDB is backed by a hashMap and it periodically dumps the contents to disk 

- So the whole storage needs to fit in memory for this to work. so this cannot be used for datasets larger than RAM. 

- if there is power failure before the data is commited to disk then it can cause potential data loss  

Disk format : 

- The db supports bool, i16, i32, i64, float32, float64, string, bytes you have to model everything to these internally 

- We id these as 0, 1, 2, ... and we store the id, key len, key, value len, value in this order so that when we are decoding from disk we can know which type and parse accordingly (I'm not sure if id is added front or back, will update this soon)

Rut learnings : 

- The difference between From and TryFrom traits
- Learnt about usage of compression 
- More will be added