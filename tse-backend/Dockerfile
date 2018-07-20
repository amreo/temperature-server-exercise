FROM ekidd/rust-musl-builder:nightly as builder
COPY tsebackend/. /home/rust/src
RUN sudo rm -r target/ || true && \ 
    sudo chmod -R a+rw . && \
    cargo build -v --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/tsebackend \
    /home/rust/src/Rocket.toml \
    /app/
EXPOSE 8000/tcp
CMD /app/tsebackend

