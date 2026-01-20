# chatfiles justfile

default:
    @just --list

# === Build ===

build:
    cargo build --release

build-debug:
    cargo build

build-web:
    cargo build --release --features web

clean *args:
    @./dev/scripts/build/clean.sh {{args}}

bloat:
    @./dev/scripts/build/bloat.sh

# === Code Quality ===

fmt *args:
    @./dev/scripts/code/fmt.sh {{args}}

lint *args:
    @./dev/scripts/code/lint.sh {{args}}

todo:
    @./dev/scripts/code/todo.sh

# === Dependencies ===

audit:
    @./dev/scripts/deps/audit.sh

outdated:
    @./dev/scripts/deps/outdated.sh

# === Testing ===

test:
    @./dev/scripts/test/quick.sh

coverage:
    @./dev/scripts/test/coverage.sh

# === Git ===

changes *args:
    @./dev/scripts/git/changes.sh {{args}}

pre-commit:
    @./dev/scripts/git/pre-commit.sh

# === Info ===

tree:
    @./dev/scripts/info/tree.sh

docs *args:
    @./dev/scripts/info/docs.sh {{args}}

# === Run ===

run *args:
    cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}
