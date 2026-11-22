# Lumo: konstruado, pruvoj kaj la loka kvalita pordego.

default: kontroli

# Rulu la unu-testojn kaj la dokumentajn testojn.
testoj:
	cargo test

# Miri (detekto de nedifinita konduto). Postulas noktan ilĉenon:
#   rustup toolchain install nightly --profile minimal --component miri --component rust-src
miri:
	cargo +nightly miri test

# Permesila kaj dependaĵa kontrolo laŭ `deny.toml` (postulas `cargo-deny`).
permesiloj:
	cargo deny check

# Komparu la pruvokatalogon kun la `#[kani::proof]`-atributoj.
pruvoj:
	bash tools/pruvoj.sh --kontroli

# Reproduktu la orajn fiksaĵojn kaj komparu ilin kun `docs/FIKSAĴOJ.md`.
haketi:
	bash tools/haketi.sh --kontroli

# Publika dokumentaro sen mankaj priskriboj.
dokumentaro:
	RUSTDOCFLAGS='-D missing-docs' cargo doc --no-deps

# Loka kvalita pordego (Q01). Kani mem ruliĝas nur en DI (Q02).
kontroli:
	bash tools/versio.sh --kontroli
	bash tools/sen-glitkomo.sh --kontroli
	bash tools/sen-cina.sh --kontroli
	bash tools/pruvoj.sh --kontroli
	bash tools/haketi.sh --kontroli
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	cargo clippy --target aarch64-unknown-none -- -D warnings
	cargo test
	RUSTDOCFLAGS='-D missing-docs' cargo doc --no-deps
	cargo build --release
