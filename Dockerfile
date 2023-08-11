# syntax=docker/dockerfile:1
# Adapted from <https://hub.docker.com/_/rust/>.

# Build:
FROM rust:1.67 as builder
WORKDIR /usr/src/proxy-solver-api
COPY proxy_solver_api .
RUN cargo install --path .

# Run:
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/proxy-solver-api /usr/local/bin/proxy-solver-api
CMD ["proxy-solver-api"]
