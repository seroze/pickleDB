# pickleDB

This is for educational & learning purpose, the original code is present at https://github.com/ninomerlino/SmollDB 

Limitations: 

- pickleDB is backed by a hashMap and it periodically dumps the contents to disk 

- So the whole storage needs to fit in memory for this to work. so this cannot be used for datasets larger than RAM. 

- if there is power failure before the data is commited to disk then it can cause potential data loss  

