perl -pi -e 's/^(# \[bench-dependencies])/substr($&, 2)/e' ./spdlog-benches/Cargo.toml
