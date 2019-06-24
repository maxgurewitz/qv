# qv server

## Deployments

The first time you deploy you will need to add a remote to your local repository.  From the `server` directory, run the following command once.

```
heroku git:remote -a shrouded-waters-56080
```

From the `qv` projects root directory, run

```
git subtree push --prefix server heroku master
```

## Local Development

### Configuring the server

Create a local `.env` file within the `server` directory. Its contents should look as follows,

```
DATABASE_URL=postgres://postgres:@localhost/qv
RUST_BACKTRACE=1
```

### Running the Server

The server can be run with vanilla cargo from the `server` directory with,

```
cargo run
```

Alternatively watch for changes with `cargo-watch`, installed with,

```
cargo install cargo-watch
```

and started by,

```
cargo watch -x run -w src
```

### Migrations

Migrations are run with the diesel library which can be installed with the following command.

```
cargo install diesel_cli --no-default-features --features postgres
```

### Testing

In order to run integration tests,

```
docker-compose --file docker-compose-local-it.yml up
```