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

test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo +nightly fmt --all -- --check

style:
	@rustup component add rustfmt 2> /dev/null
	cargo install taplo-cli 2> /dev/null
	cargo +nightly fmt --all && taplo fmt

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

udeps:
	SKIP_WASM_BUILD=1 cargo +nightly udeps -q --all-targets

dev:
	cargo run

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

containerize-mmr-polkadot:
	@docker build -f docker/mmr-polkadot.dockerfile \
		-t ${REPO}/mmr-polkadot:latest  \
		.

push-mmr-polkadot:
	@docker push ${REPO}/mmr-polkadot:latest

containerize-ci-linux:
	@docker build -f docker/ci-linux.dockerfile \
		-t ${REPO}/ci-linux:production  \
		.

push-ci-linux:
	@docker push ${REPO}/ci-linux:production

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
.PHONY: containerize-composable-sandbox push-composable-sandbox containerize-mmr-polkadot push-mmr-polkadot
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
	make vendor                  : Download dependencies into the 'vendor' folder. \n\
	make containerize            : Bundle the compiled binary in a lean production-ready docker image. \n\
	make install                 : Use docker-compose to startup composable alongside other needed services
	make stop				     : Stop all current running containers
	make push				     : Push all built images to the specified docker registry
"
endef
