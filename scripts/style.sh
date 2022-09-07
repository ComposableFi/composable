#!/usr/bin/env bash

# TODO: remove version as must run under just +nightly after nix env applied
NIGHTLY_VERSION="2022-04-18"

usage() {
    cat <<EOF
Formats all the code in the repository.

usage: style.sh [options]

Options:
    -h, --help      Shows this dialogue
    -c, --check     Check only, exiting with a non-zero exit code if not
                    formatted correctly
    -v, --verbose   Use verbose output
EOF
}

# Global configuration variables, read by all the formatting functions
check=""
verbose=""

check_return_code() {
    if [ $? -eq 0 ]; then
        echo "OK"
    fi
}

cargo_fmt() {
    rustfmt_check=""
    rustfmt_verbose=""

    if [[ ${check} = "check" ]]; then
        rustfmt_check="-- --check"
    fi

    if [[ ${verbose} = "verbose" ]]; then
        rustfmt_verbose="--verbose"
    fi

    echo "Running rustfmt..."

    cargo +nightly-${NIGHTLY_VERSION} fmt --all ${rustfmt_verbose} ${rustfmt_check}

    check_return_code
}

taplo_fmt() {
    taplo_verbose=""
    if [[ ${verbose} = "verbose" ]]; then
        taplo_verbose="--verbose"
    fi

    echo "Running taplo..."

    if [[ ${check} = "check" ]]; then
        taplo check ${taplo_verbose}
    else
        taplo fmt ${taplo_verbose}
    fi

    check_return_code
}

prettier_fmt() {
    # cd integration-tests/runtime-tests
    prettier_verbose=""

    if [[ ${verbose} = "verbose" ]]; then
        prettier_verbose="--loglevel=debug"
    else
        prettier_verbose="--loglevel=warn"
    fi

    echo "Running Prettier on integration-tests/runtime-tests..."

    if [[ ${check} = "check" ]]; then
        npx prettier \
            --config="integration-tests/runtime-tests/.prettierrc" \
            --ignore-path="integration-tests/runtime-tests/.prettierignore" \
            --check ${prettier_verbose} \
            "integration-tests/runtime-tests/"
    else
        npx prettier \
            --config="integration-tests/runtime-tests/.prettierrc" \
            --ignore-path="integration-tests/runtime-tests/.prettierignore" \
            --write ${prettier_verbose} \
            "integration-tests/runtime-tests/"
    fi

    check_return_code
}

# install taplo if it isn't already
maybe_taplo=$(whereis taplo)
if [[ ${maybe_taplo} = "taplo: " ]]; then
    cargo install taplo-cli 2>/dev/null
fi

for arg in "$@"; do
    case $arg in
    "--help" | "-h")
        usage
        exit 0
        ;;
    "--check" | "-c")
        check="check"
        ;;
    "--verbose" | "-v")
        verbose="verbose"
        ;;
    *)
        echo "Unknown option '$arg'"
        usage
        exit 1
        ;;
    esac
done

cargo_fmt
taplo_fmt
prettier_fmt
