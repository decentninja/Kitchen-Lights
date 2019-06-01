export OPENSSL_LIB_DIR=/home/ubuntu/Kitchen-Lights/openssl-1.0.1t 
export OPENSSL_INCLUDE_DIR=/home/ubuntu/Kitchen-Lights/openssl-1.0.1t/include
mkdir .cargo
cat > .cargo/config << EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
cargo build --target armv7-unknown-linux-gnueabihf --release
