FROM immunant/c2rust:ubuntu-focal-latest

# build and install c2rust
ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config
RUN git clone https://github.com/immunant/c2rust.git
WORKDIR c2rust
RUN cargo build --release || true
RUN cargo build --release || true
RUN cp ./target/release/c2rust* /usr/bin

# copy transpiler
WORKDIR /transpiler
COPY . ./