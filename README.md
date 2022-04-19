# risingwave-poc
demo program for POC

## How to test
1. Installing kafka through docker-compose 
```
cd $RISINGWAVE_POC/docker-compose
docker-compose -f docker-compose.yaml up
```
2. Create topic for test
```
 docker exec -it broker  /usr/bin/kafka-topics --create --topic test_topic --bootstrap-server broker:9092 --partitions 3
```
3. Build the executable bench program
```
cargo build --bin bench --release
cd target/release
 ./bench --conf CONIFG_PATH
```