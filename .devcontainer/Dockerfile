# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.177.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends docker.io

#inform cross that it is running inside another docker container
ENV CROSS_DOCKER_IN_DOCKER=true


#Create dummy socket file so mounting of docker.sock will not create directory instead of socket file
RUN touch /var/run/docker.sock \
    && chown root:docker /var/run/docker.sock \
    && usermod -aG docker vscode

#install cross
RUN cargo install cross

#set alias for cross build debug
RUN echo 'alias cbd="cross build --target armv7-unknown-linux-gnueabihf"' >> /home/vscode/.bashrc

#set alias for cross build release
RUN echo 'alias cbr="cross build --release --target armv7-unknown-linux-gnueabihf"' >> /home/vscode/.bashrc

#add cargo upload command to path
RUN mkdir /home/vscode/bin && echo 'export PATH="/home/vscode/bin:$PATH"' >> /home/vscode/.bashrc
COPY ./scripts/cargo-upload /home/vscode/bin/cargo-upload
RUN chown vscode /home/vscode/bin/cargo-upload && chmod +x /home/vscode/bin/cargo-upload

# fix permissions before starting container, keep open using tail /dev/null
CMD ["sh", "-c", "sudo chown -R vscode /usr/local/cargo/ && sudo usermod -aG docker vscode && tail -f /dev/null"]