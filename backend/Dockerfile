####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y pkg-config make g++ libssl-dev 
RUN rustup target add x86_64-unknown-linux-musl
RUN update-ca-certificates

WORKDIR /backend

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

WORKDIR /backend

COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/backend ./

EXPOSE 8080

CMD ["/backend/backend"]