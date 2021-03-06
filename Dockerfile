FROM rust:1-stretch

RUN apt-get update && apt-get install -y --no-install-recommends \
    gettext \
    postgresql-client \
    libpq-dev \
    git \
    curl \
    gcc \
    make \
    openssl \
    libssl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo install diesel_cli --no-default-features --features postgres --version '=1.3.0'
COPY . .
RUN cargo install --force --no-default-features --features postgres
RUN cargo install --path plume-cli --force --no-default-features --features postgres
RUN rm -rf target/debug/incremental
CMD ["plume"]
EXPOSE 7878
