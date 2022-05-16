FROM rust as builder

ADD ./workload-generator /workload-generator-src
RUN apt update && apt install -y cmake
RUN cd /workload-generator-src && cargo build --release

FROM ubuntu:20.04

ADD ./ad-click /ad-click
ADD ./ad-ctr /ad-ctr

COPY --from=builder /workload-generator-src/target/release/workload_generator /
ENTRYPOINT [ "/workload_generator" ]
