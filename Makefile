COMMIT_SHA:=$(shell git rev-parse --short=9 HEAD)
BRANCH_NAME:=$(shell git rev-parse --abbrev-ref HEAD | tr '/' '-')
REPO=composablefi
SERVICE_NAME=composable
INSTALL_DIR=docker/
IMAGE_URL:=${REPO}/${SERVICE_NAME}
AUTO_UPDATE:=1


IMAGE?=${IMAGE_URL}:${COMMIT_SHA}
IMAGE_WITH_COMMIT=${IMAGE}
IMAGE_WITH_RELEASE_VERSION:=${IMAGE_URL}:${RELEASE_VERSION}
IMAGE_WITH_BRANCH:=${IMAGE_URL}:${BRANCH_NAME}
IMAGE_WITH_LATEST:=${IMAGE_URL}:latest

help:
	@echo $(print_help_text)

build:
	@cargo build

clean:
	@cargo clean

.PHONY: build-release
build-release:
	cargo build --locked --features with-all-runtime --profile production --workspace --exclude runtime-integration-tests --exclude e2e-tests --exclude test-service

bench:
	./scripts/benchmark.sh

test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	./scripts/style.sh --check --verbose

style:
	@rustup component add rustfmt 2> /dev/null
	./scripts/style.sh

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

udeps:
	SKIP_WASM_BUILD=1 cargo +nightly udeps -q --all-targets

dev:
	cargo run

# run as `make open=y run-book` to open as well
run-book:
	bash -c "(trap 'kill 0' SIGINT; cargo run --manifest-path utils/extrinsics-docs-scraper/Cargo.toml --release -- --config-file-path=scraper.toml -vvv --watch & mdbook serve --hostname 0.0.0.0 book/ $(if $(filter y,${open}),'--open'))"

build-book:
	cargo run --manifest-path utils/extrinsics-docs-scraper/Cargo.toml --release -- --config-file-path=scraper.toml
	mdbook build book/

.PHONY: version
version:
	@if [ ${RELEASE_VERSION} ]; then \
	sed -i "s|^version =.*|version = '"${RELEASE_VERSION}"'|" node/Cargo.toml; \
	fi;


.PHONY: containerize-release
containerize-release: version containerize

containerize:
	@docker build \
	--build-arg SERVICE_DIR=${INSTALL_DIR} --build-arg VERSION=${RELEASE_VERSION} \
		-f ${INSTALL_DIR}/Dockerfile \
		-t ${IMAGE_WITH_COMMIT} \
		-t ${IMAGE_WITH_RELEASE_VERSION} \
		-t ${IMAGE_WITH_BRANCH} \
		-t ${IMAGE_WITH_LATEST} \
	. 1>/dev/null

push:
	@docker push ${IMAGE_WITH_COMMIT}
	@docker push ${IMAGE_WITH_BRANCH}
	@docker push ${IMAGE_WITH_RELEASE_VERSION}
	@docker push ${IMAGE_WITH_LATEST}

push-release:
	@docker push ${IMAGE_WITH_RELEASE_VERSION}

containerize-composable-sandbox:
	@docker build -f docker/composable-sandbox.dockerfile \
		-t ${REPO}/composable-sandbox:${COMMIT_SHA} \
		-t ${REPO}/composable-sandbox:latest  \
		.

push-composable-sandbox:
	@docker push ${REPO}/composable-sandbox:${COMMIT_SHA}
	@docker push ${REPO}/composable-sandbox:latest

push-composable-sandbox-without-latest-tag:
	@docker push ${REPO}/composable-sandbox:${COMMIT_SHA}

containerize-composable-sandbox-plus:
	@docker build -f docker/composable-sandbox-plus.dockerfile \
		-t ${REPO}/composable-sandbox-plus:${COMMIT_SHA} \
		-t ${REPO}/composable-sandbox-plus:latest  \
		.

push-composable-sandbox-plus:
	@docker push ${REPO}/composable-sandbox-plus:${COMMIT_SHA}
	@docker push ${REPO}/composable-sandbox-plus:latest

containerize-mmr-polkadot:
	@docker build -f docker/mmr-polkadot.dockerfile \
		-t ${REPO}/mmr-polkadot:latest  \
		.

push-mmr-polkadot:
	@docker push ${REPO}/mmr-polkadot:latest

containerize-ci-linux:
	@docker build -f docker/ci-linux.dockerfile \
		-t ${REPO}/ci-linux:2022-04-18  \
		.

push-ci-linux:
	@docker push ${REPO}/ci-linux:2022-04-18

containerize-base-ci-linux:
	@docker build -f docker/base-ci-linux.dockerfile \
		-t ${REPO}/base-ci-linux:1.60.0  \
		.

push-base-ci-linux:
	@docker push ${REPO}/base-ci-linux:1.60.0

stop:
	@docker-compose down

install:
		$(info Run if auto-update is enabled)
ifeq ($(AUTO_UPDATE),1)
	docker-compose up
else
		$(info Auto-Update disabled, please use docker-run to start this project)
endif


.PHONY: build test docs style-check lint udeps containerize dev push install stop containerize-release push-release
.PHONY: containerize-composable-sandbox push-composable-sandbox push-composable-sandbox-without-latest-tag
.PHONY: containerize-composable-sandbox-plus push-composable-sandbox-plus
.PHONY: containerize-mmr-polkadot push-mmr-polkadot
.PHONY: containerize-base-ci-linux push-base-ci-linux
.PHONY: containerize-ci-linux push-ci-linux

#----------------------------------------------------------------------
# UTILITY FUNCTIONS TO remove
#----------------------------------------------------------------------

define _info
	echo "$$(tput setaf 2) ${1} $$(tput sgr0)"
endef

define _error
	echo "$$(tput setaf 1) ${1} $$(tput sgr0)"
endef

define print_help_text
"Here are the commands to help setting up composable in any environment: \n\
	--- Dev --- \n\
	make help                    : Display this help message. \n\
	make containerize            : Bundle the compiled binary in a lean production-ready docker image. \n\
	make install                 : Use docker-compose to startup composable alongside other needed services
	make stop				     : Stop all current running containers
	make push				     : Push all built images to the specified docker registry
"
endef
