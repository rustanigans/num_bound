FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
USER vscode
RUN /usr/local/cargo/bin/rustup install nightly
RUN /usr/local/cargo/bin/rustup component add --toolchain nightly rustfmt
RUN /usr/local/cargo/bin/cargo install cargo-cache
