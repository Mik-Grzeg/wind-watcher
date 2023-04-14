set shell := ["zsh", "-uc"]

default:
  @just --choose

# starts docker containers and builds local images
up *args:
  docker compose up -d --build {{args}}

# stops docker containers
down *args:
  docker compose down {{args}}

# Clean all containers and their volumes
clean:
  docker compose down -v

# show docker compose ps
ps: 
  docker compose ps 

# logs of a container
@logs SERVICE *args:
  docker compose logs {{args}} {{SERVICE}}

# Run cargo fmt for a given service
@fmt SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo fmt {{args}}
  echo All good!

# Run cargo clippy for a given service
@clippy SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo clippy {{args}}
  echo All good!

# Run cargo check for a given service
@check SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo check {{args}}

# Run given service
@run SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo run {{args}}

# Build given service
@build SERVICE *args:
  cd services/{{SERVICE}} || cd {{SERVICE}} && cargo build {{args}}
