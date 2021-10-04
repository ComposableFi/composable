COMMIT_SHA:=$(shell git rev-parse --short=9 HEAD)
BRANCH_NAME:=$(shell git rev-parse --abbrev-ref HEAD | tr '/' '-')
REPO=composablefi
SERVICE_NAME=composable
INSTALL_DIR=install/docker
IMAGE_URL:=${REPO}/${SERVICE_NAME}


IMAGE?=${IMAGE_URL}:${COMMIT_SHA}
IMAGE_WITH_COMMIT=${IMAGE}
IMAGE_WITH_BRANCH:=${IMAGE_URL}:${BRANCH_NAME}
IMAGE_WITH_LATEST:=${IMAGE_URL}:latest

build:
	@cargo build

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run

containerize:
	@docker build \
	--build-arg SERVICE_DIR=${INSTALL_DIR} \
       	-f ${INSTALL_DIR}/Dockerfile \
       	-t ${IMAGE_WITH_COMMIT} \
        -t ${IMAGE_WITH_BRANCH} \
        -t ${IMAGE_WITH_LATEST} \
	. 1>/dev/null

push:
	@docker push ${IMAGE_WITH_COMMIT}
	@docker push ${IMAGE_WITH_BRANCH}
	@docker push ${IMAGE_WITH_LATEST}

up:
	@docker-compose up

down:
	@docker-compose down


.PHONY: build test docs style-check lint rust-image up down containerize dev


#----------------------------------------------------------------------
# UTILITY FUNCTIONS
#----------------------------------------------------------------------

define _info
	echo "$$(tput setaf 2) ${1} $$(tput sgr0)"
endef

define _error
	echo "$$(tput setaf 1) ${1} $$(tput sgr0)"
endef

define print_help_text
"Here are the commands you can use for ${SERVICE_LONG_NAME}: \n\
	--- Dev --- \n\
	make help                    : Display this help message. \n\
	make vendor                  : Download dependencies into the 'vendor' folder. \n\
"
endef
