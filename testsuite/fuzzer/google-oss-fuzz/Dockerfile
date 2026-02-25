FROM gcr.io/oss-fuzz-base/base-builder-rust
RUN apt-get update && apt-get install -y make autoconf automake libclang-dev libtool pkg-config
RUN git clone --depth=1 https://github.com/creditchainorg/creditchain-core.git
WORKDIR creditchain-core
COPY build.sh $SRC
