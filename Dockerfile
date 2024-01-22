# syntax=docker/dockerfile:1
# Adapted from <https://hub.docker.com/_/rust/>.

# Build:
FROM rust:1.67 as builder
WORKDIR /usr/src/proxy-solver-api
COPY . .
RUN cargo install --all-features --path .

# Run:
FROM nginx:latest
COPY --from=builder /usr/local/cargo/bin/proxy-solver-api /usr/local/bin/proxy-solver-api
COPY nginx.conf /etc/nginx/templates/default.conf.template
COPY entrypoint.sh /docker-entrypoint.d/99-entrypoint.sh
