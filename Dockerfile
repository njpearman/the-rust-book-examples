FROM ubuntu:18.04

RUN apt-get update -yqq && apt-get install -yqq --no-install-recommends \
  build-essential \
  curl \
  ca-certificates

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH "/root/.cargo/bin:$PATH"
ENV USER "workbench"

RUN rustup update

WORKDIR /usr/src/workbench

CMD ["bin/bash"]
