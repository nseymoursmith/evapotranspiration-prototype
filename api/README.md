# How do I run this?

You will need `docker` and `docker-compose`. It should be a simple case of
using:
* `docker-compose up --build -d`

Where:

* `docker-compose` is the runtime
* `up` is the command (which looks for a `docker-compose.yaml` in the same
  folder`
* `--build` guarantees that docker re-builds the project - docker is very lazy
  and will usually build with caches wherever possible
* `-d` is for 'detach` - runs the docker in the background.

`docker ps` should show something like:

```
CONTAINER ID   IMAGE                         COMMAND                  CREATED          STATUS          PORTS
           NAMES
af8e6403610e   dockerised-rust-api-simple-api   "./target/release/ap…"   0 minutes ago   Up 0 minutes   0.0.0.0:3030->3030/tcp, :::3030-> 3030/tcp   simple-api
```

# What does it do?

See `Dockerfile` - installs the elements from the `Cargo.toml` and builds the
project, serving it on port `3030` (see `./src/main.rs`).

Creates and runs and API on port `3030` which is then passed from the container
to the host - so the app is visible externally at `localhost:3030`

It consists of two routes - `/` and `/test_route` - check with `curl localhost/`
or `curl localhost/test_route`

