FROM composablefi/composable-devcontainer:latest
#FROM composable-codespace:s6rgaf5isy6n5zih6azbcv2119jyrgrh

# TODO: make fetchTarbal stuff easy

# # Add polkadot
# Not straight to the workspace dir because that seems to be causing issues.
# We move it in the devcontainer.json's postCreateCommand.
# WORKDIR /polkadot-binary
# RUN wget "https://github.com/paritytech/polkadot/releases/download/v0.9.24/polkadot" && \
#     chmod +x polkadot


# # Add mdbook
# WORKDIR /mdbook-binary
# RUN wget "https://github.com/rust-lang/mdBook/releases/download/v0.4.18/mdbook-v0.4.18-x86_64-unknown-linux-gnu.tar.gz" && \ 
#     tar -xzf mdbook-v0.4.18-x86_64-unknown-linux-gnu.tar.gz && \
#     chmod +x mdbook && \
#     cp ./mdbook /usr/bin 

# # Add taplo
# WORKDIR /taplo-binary
# RUN wget "https://github.com/tamasfe/taplo/releases/download/release-cli-0.6.2/taplo-0.6.2-x86_64-unknown-linux-gnu.tar.gz" && \
#     tar -xzf taplo-0.6.2-x86_64-unknown-linux-gnu.tar.gz && \
#     chmod +x taplo && \
#     cp ./taplo /usr/bin
