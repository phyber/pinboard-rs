CARGO ?= cargo
CARGO_OPTS ?=
CARGO_WITH_OPTS ?= $(CARGO) $(CARGO_OPTS)
CARGO_BUILD_OPTS ?=
CARGO_BUILD ?= $(CARGO_WITH_OPTS) build $(CARGO_BUILD_OPTS)
CARGO_BUILD_RELEASE ?= $(CARGO_BUILD) --release
CARGO_CLEAN_OPTS ?=
CARGO_CLEAN ?= $(CARGO_WITH_OPTS) clean $(CARGO_CLEAN_OPTS)
CARGO_TEST_OPTS ?=
CARGO_TEST ?= $(CARGO_WITH_OPTS) test $(CARGO_TEST_OPTS)

# This is a temporary workaround to get the openssl crates to compile.
OPENSSL_DIR ?= /usr
.export OPENSSL_DIR

build:
	$(CARGO_BUILD)

release:
	$(CARGO_BUILD_RELEASE)

clean:
	$(CARGO_CLEAN)

test:
	$(CARGO_TEST)
