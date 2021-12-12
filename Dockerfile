#!/bin/bash
# docker build -t apache/iotdb-cli:latest  .

FROM --platform=linux/amd64 rust as builder

WORKDIR iotdb-cli

COPY . .

RUN cargo build --release --bin iotdb

FROM apache/iotdb as runtime

COPY --from=builder /iotdb-cli/target/release/iotdb /usr/local/bin