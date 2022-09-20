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
	echo "This command is deprecated. Please use \`nix build \".#composable-node\"\` instead."
	return 1

clean:
	cd code
	@cargo clean

release:
	echo "This command is deprecated. Please use \`nix build \".#composable-node\"\` instead."
	return 1

.PHONY: build-release
build-release:
	cd code
	cargo build --locked --features with-all-runtime --profile production --workspace --exclude runtime-integration-tests --exclude e2e-tests --exclude test-service

bench:
	./scripts/benchmark.sh

test:
	echo "This command is deprecated. Please use \`nix run \".#unit-tests\"\` instead."
	return 1

docs: build
	cd code
	@cargo doc --no-deps

style-check:
	echo "This command is deprecated. Please use checks defined in flake.nix instead."
	return 1

style:
	echo "This command is deprecated. Please use \`nix run \".#fmt\"\` instead."
	return 1

lint:
	@rustup component add clippy 2> /dev/null
	cd code
	cargo clippy --all-targets --all-features -- -D warnings

udeps:
	cd code
	SKIP_WASM_BUILD=1 cargo +nightly udeps -q --all-targets

# run as `make open=y run-book` to open as well
run-book:
	bash -c "(trap 'kill 0' SIGINT; cargo run --manifest-path code/utils/extrinsics-docs-scraper/Cargo.toml --release -- --config-file-path=scraper.toml -vvv --watch & mdbook serve --hostname 0.0.0.0 book/ $(if $(filter y,${open}),'--open'))"

build-book:
	cargo run --manifest-path code/utils/extrinsics-docs-scraper/Cargo.toml --release -- --config-file-path=scraper.toml
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

containerize-mmr-polkadot:
	@docker build -f docker/mmr-polkadot.dockerfile \
		-t ${REPO}/mmr-polkadot:latest  \
		.

push-mmr-polkadot:
	@docker push ${REPO}/mmr-polkadot:latest

containerize-lease-period-prolongator:
	@docker build -f scripts/lease-period-prolongator/Dockerfile \
		-t ${REPO}/lease-period-prolongator:0.1.0  \
		scripts/lease-period-prolongator

push-lease-period-prolongator:
	@docker push ${REPO}/lease-period-prolongator:0.1.0

install:
		$(info Run if auto-update is enabled)
ifeq ($(AUTO_UPDATE),1)
	docker-compose up
else
		$(info Auto-Update disabled, please use docker-run to start this project)
endif


.PHONY: build test docs style-check lint udeps containerize dev push install stop containerize-release push-release
.PHONY: containerize-mmr-polkadot push-mmr-polkadot
.PHONY: containerize-lease-period-prolongator push-lease-period-prolongator

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
