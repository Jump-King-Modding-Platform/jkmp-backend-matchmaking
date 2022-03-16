FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=jkmp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /jkmp

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /jkmp

COPY --from=builder /jkmp/target/x86_64-unknown-linux-musl/release/server ./

USER jkmp:jkmp

CMD ["/jkmp/server", "--port", "16000"]