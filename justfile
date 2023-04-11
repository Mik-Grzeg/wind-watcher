set shell := ["zsh", "-uc"]

default:
  @just --choose


# starts docker containers and builds local images
up:
  docker compose up -d --build

# stops docker containers
down *args:
  docker compose down {{args}}

# show docker compose ps
dps: 
  docker compose ps 

# logs of a container
@logs SERVICE *args:
  docker compose logs {{args}} {{SERVICE}}

# Run cargo fmt for a given service
@fmt SERVICE:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo fmt
  echo All good!

# Run cargo clippy for a given service
@clippy SERVICE:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo clippy
  echo All good!

# Run cargo build for a given service
@check SERVICE:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo check

# Run given service
@run SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo run {{args}}

