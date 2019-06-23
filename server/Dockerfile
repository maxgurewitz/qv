FROM rust

RUN cargo install cargo-watch

# Creating a directory to work from
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Copy our app into that directory
COPY . /usr/src/app
RUN cargo fetch

# Build our app
VOLUME ["/usr/local/cargo"]
