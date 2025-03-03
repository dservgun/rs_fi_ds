FROM rust:1.84.0 as builder

WORKDIR .

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12


COPY --from=builder ./target/release/rs_fi_ds /usr/local/bin/rs_fi_ds

CMD ["rs_fi_ds"]