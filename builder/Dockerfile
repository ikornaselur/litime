FROM rust:1.59-alpine

# Add regular dependencies
RUN apk add --no-cache curl jq git build-base bash zip

# Add windows dependencies
RUN apk add --no-cache mingw-w64-gcc

# Add apple dependencies
RUN apk add --no-cache clang cmake libxml2-dev openssl-dev fts-dev bsd-compat-headers xz
RUN git clone https://github.com/tpoechtrager/osxcross /opt/osxcross
RUN curl -Lo /opt/osxcross/tarballs/MacOSX10.10.sdk.tar.xz "https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz"
RUN ["/bin/bash", "-c", "cd /opt/osxcross && UNATTENDED=yes OSX_VERSION_MIN=10.8 ./build.sh"]

COPY entrypoint.sh ./entrypoint.sh
COPY build.sh ./build.sh

RUN chmod +x /entrypoint.sh /build.sh

ENTRYPOINT ["/entrypoint.sh"]
