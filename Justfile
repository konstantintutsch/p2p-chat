[private]
default:
    just --list --justfile {{ justfile() }}

test-cert:
    mkcert 0.0.0.0 localhost 127.0.0.1 ::1
    mkcert -install

test-run:
    cargo run -- 0.0.0.0+3.pem 0.0.0.0+3-key.pem
