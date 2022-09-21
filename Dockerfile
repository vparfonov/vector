FROM registry.redhat.io/ubi8:8.6-943 as builder

RUN INSTALL_PKGS=" \
      cmake \
      gcc-c++ \
      libarchive \	  
      make \
      git \
      openssl-devel \
      llvm-toolset \
      cyrus-sasl \
      python36 \
      llvm \
      cyrus-sasl-devel \
      libtool \
      " && \
    yum install -y $INSTALL_PKGS && \
    rpm -V $INSTALL_PKGS && \
    yum clean all

ENV HOME=/root
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.61.0 -y
ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN mkdir -p /src

WORKDIR /src
COPY . /src

RUN PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch)  make build


FROM registry.redhat.io/ubi8:8.6-943

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
