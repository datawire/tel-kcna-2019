#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

if ! [ -x "$(command -v kubeapply)" ]; then
    echo "Error: kubeapply is not installed"
    exit 1
fi
kubeapply -f k8s
