#! /usr/bin/env bash

NIGHTLY_VERSION="2022-04-18"

cargo_fmt() {
    rustfmt_check=""
    rustfmt_verbose=""

    if [[ $1 = "check" ]]; then
        rustfmt_check="-- --check"
    fi

    if [[ $2 = "verbose" ]]; then
        rustfmt_verbose="--verbose"
    fi

    cargo +nightly-${NIGHTLY_VERSION} fmt --all ${rustfmt_verbose} ${rustfmt_check}
}

taplo_fmt() {
    taplo_verbose=""
    if [[ $2 = "verbose" ]]; then
        taplo_verbose="--verbose"
    fi

    if [[ $1 = "check" ]]; then
        taplo check ${taplo_verbose}
    else
        taplo fmt ${taplo_verbose}
    fi
}

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

# install taplo if it isn't already
maybe_taplo=$(whereis taplo)
if [[ ${maybe_taplo} = "taplo: " ]]; then
    cargo install taplo-cli 2>/dev/null
fi

check=""
verbose=""

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

cargo_fmt ${check} ${verbose}
taplo_fmt ${check} ${verbose}
