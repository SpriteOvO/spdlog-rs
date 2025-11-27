just := 'just' + ' --justfile=' + justfile()

_:
    @{{ just }} --list

alias format := fmt

fmt *ARGS:
    cargo +nightly fmt --all {{ ARGS }}

test *ARGS:
    cargo test \
      --features 'log native libsystemd multi-thread runtime-pattern serde serde_json sval' \
      --features 'flexible-string source-location std-stream-captured' \
      {{ ARGS }}

clippy *ARGS:
    cargo clippy --all-features --tests --examples {{ ARGS }}

check *ARGS:
    cargo check --all-features --tests --examples {{ ARGS }}

_doc-default-features *ARGS:
    cargo +nightly doc -Z unstable-options -Z rustdoc-scrape-examples {{ ARGS }}

doc *ARGS:
    @{{ just }} _doc-default-features --all-features {{ ARGS }}

bench *ARGS:
    cargo +nightly bench --features 'multi-thread runtime-pattern serde_json log' {{ ARGS }}

[private]
publish crate-name *ARGS:
    cargo clean
    cargo package --package {{ crate-name }}
    @{{ just }} _publish-confirmed {{ crate-name }} {{ ARGS }}

[confirm("Please check 'target/package' directory, keep going? (y/N)")]
_publish-confirmed crate-name *ARGS:
    cargo publish --package {{ crate-name }} {{ ARGS }}
