CARGO ?= cargo
CARGO_OPTS ?=
CARGO_BUILD_OPTS ?=
CARGO_BUILD ?= $(CARGO) $(CARGO_OPTS) build $(CARGO_BUILD_OPTS)
CARGO_BUILD_RELEASE = $(CARGO_BUILD) --release
CARGO_CLEAN_OPTS ?=
CARGO_CLEAN =? $(CARGO) $(CARGO_OPTS) clean $(CARGO_CLEAN_OPTS)

# This is a temporary workaround to get the openssl crates to compile.
OPENSSL_DIR ?= /usr
.export OPENSSL_DIR

build:
	$(CARGO_BUILD)

release:
	$(CARGO_BUILD_RELEASE)

clean:
	$(CARGO_CLEAN)
