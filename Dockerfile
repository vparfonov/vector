FROM registry-proxy.engineering.redhat.com/rh-osbs/ubi8:8.7-626 as builder

RUN rm /etc/yum.repos.d/*
ADD 8.7-compose.repo /etc/yum.repos.d

RUN INSTALL_PKGS=" \
      rust-toolset \
      gcc-c++ \
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

RUN PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch)  make build


FROM registry-proxy.engineering.redhat.com/rh-osbs/ubi8:8.7-626

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
