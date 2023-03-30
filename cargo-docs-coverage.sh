#!/bin/bash

# docs coverage generator thing

if [ "$(rustup default | grep -c nightly)" -eq 0 ]; then
    export NEEDS_NIGHTLY=1
    echo "***************************" >&2
    echo "*** SWAPPING TO NIGHTLY ***" >&2
    echo "***************************" >&2
    rustup default nightly >&2
else
    export NEEDS_NIGHTLY=0
fi


RUSTDOCFLAGS='-Z unstable-options --show-coverage --output-format json' cargo doc -q --no-deps

if [ "${NEEDS_NIGHTLY}" -eq 1 ]; then

    echo "*******************************" >&2
    echo "*** SWAPPING BACK TO STABLE ***" >&2
    echo "*******************************" >&2
    rustup default stable >&2
fi