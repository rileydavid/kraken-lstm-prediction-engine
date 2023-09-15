####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y pkg-config make g++ libssl-dev 
RUN rustup target add x86_64-unknown-linux-musl
RUN update-ca-certificates

WORKDIR /converter

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

WORKDIR /

COPY --from=builder /converter/target/x86_64-unknown-linux-musl/release/converter ./

RUN mkdir data

CMD ["/converter"]