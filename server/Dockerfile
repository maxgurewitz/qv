FROM rust

RUN cargo install cargo-watch

# Creating a directory to work from
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Copy our app into that directory
COPY . /usr/src/app
COPY wait-for.sh /usr/local/bin
RUN chmod +x /usr/local/bin/wait-for.sh
RUN cargo fetch

# Build our app
VOLUME ["/usr/local/cargo"]
