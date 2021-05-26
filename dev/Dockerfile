FROM rust:1-buster

# add vsc server install script
COPY scripts/install_vsc_server.sh /scripts/install_vsc_server.sh

# add non-root 'code' user
RUN useradd --user-group --create-home --shell /bin/bash code
USER code
WORKDIR /home/code

# install vsc server as code user
RUN /scripts/install_vsc_server.sh

CMD tail -f /dev/null
