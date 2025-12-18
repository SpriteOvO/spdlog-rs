just := 'just' + ' --justfile=' + justfile()

#

additive-features := 'log native libsystemd multi-thread runtime-pattern serde serde_json sval'
selective-features := 'flexible-string source-location std-stream-captured'
test-features := additive-features + ' ' + selective-features

_:
    @{{ just }} --list

alias format := fmt

fmt *ARGS:
    cargo +nightly fmt --all {{ ARGS }}

test *ARGS:
    cargo test --features '{{ test-features }}' {{ ARGS }}

clippy *ARGS:
    cargo clippy --all-features --tests --examples {{ ARGS }}

check *ARGS:
    cargo check --all-features --tests --examples {{ ARGS }}

_doc-default-features *ARGS:
    cargo +nightly doc \
        --workspace --exclude spdlog-macros --exclude spdlog-internal \
        -Z unstable-options -Z rustdoc-scrape-examples {{ ARGS }}

doc *ARGS:
    @{{ just }} _doc-default-features --all-features {{ ARGS }}

bench *ARGS:
    cargo +nightly bench --features 'multi-thread runtime-pattern serde_json log' {{ ARGS }}

miri cmd *ARGS:
    MIRIFLAGS='-Zmiri-disable-isolation' cargo +nightly miri {{ cmd }} --features '{{ test-features }}' {{ ARGS }}

[private]
publish crate-name *ARGS:
    cargo clean
    cargo package --package {{ crate-name }}
    @{{ just }} _publish-confirmed {{ crate-name }} {{ ARGS }}

[confirm("Please check 'target/package' directory, keep going? (y/N)")]
_publish-confirmed crate-name *ARGS:
    cargo publish --package {{ crate-name }} {{ ARGS }}
