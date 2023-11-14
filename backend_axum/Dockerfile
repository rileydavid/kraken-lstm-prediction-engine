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

COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/server ./
#copying the .env file for docker 
COPY --from=builder /backend/env/.env ./

EXPOSE 8000

CMD ["/backend/server"]