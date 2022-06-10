FROM registry.redhat.io/ubi8:8.6-754 as builder

RUN INSTALL_PKGS=" \
      rust-toolset \
      cmake \
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

RUN mkdir -p /src

WORKDIR /src
COPY . /src
ENV PROTOC /src/protoc/protoc

RUN make build


FROM registry.redhat.io/ubi8:8.6-754

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
