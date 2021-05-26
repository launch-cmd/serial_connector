#!/bin/sh
set -e

# You can get the latest commit SHA by looking at the latest tagged commit here: https://github.com/microsoft/vscode/releases
commit_sha="054a9295330880ed74ceaedda236253b4f39a335"
archive="vscode-server-linux-x64.tar.gz"

echo "downloading latest VS Code Server version"

# Download VS Code Server tarball to tmp directory.
curl -L "https://update.code.visualstudio.com/latest/server-linux-x64/stable" -o "/tmp/${archive}"

# Extract product file in tmp folder to find commit_sha
tar xvf "/tmp/${archive}" --strip-components=1 -C /tmp/ vscode-server-linux-x64/product.json

# Set commit sha
commit_sha=$(cat /tmp/product.json | python -c \
    'import json,sys;print json.load(sys.stdin)["commit"]')
echo "found vscode server sha: ${commit_sha}"

# Make the parent directory where the server should live.
# NOTE: Ensure VS Code will have read/write access; namely the user running VScode or container user.
mkdir -p ~/.vscode-server/bin/"${commit_sha}"

# Extract the tarball to the right location.
tar --no-same-owner -xz --strip-components=1 -C ~/.vscode-server/bin/"${commit_sha}" -f "/tmp/${archive}"

# Add 0 file
touch ~/.vscode-server/bin/${commit_sha}/0

# Install needed extensions
~/.vscode-server/bin/"${commit_sha}"/server.sh --force-disable-user-env --use-host-proxy --port 0 --extensions-download-dir ~/.vscode-server/extensionsCache --extensions-dir ~/.vscode-server/extensions --install-extension rust-lang.rust --install-extension bungcip.better-toml
