# build
echo "building"
export ZIG_GLOBAL_CACHE_DIR=$PWD/target
RUSTFLAGS="-C link-arg=-ldl" cargo zigbuild --target=armv7-unknown-linux-musleabi --release --bin lifeguard 

# clear cobwebs (idk why this is needed - mounting maybe?)
ssh root@192.168.0.252 "echo 'aaaachooo - alright cleared cobwebs!'"

# transfer
echo "transferring"
scp target/armv7-unknown-linux-musleabi/release/lifeguard root@192.168.0.252:/mnt/us/dev/lifeguard

# run
echo "running"
# note; -t needed to create a pseudo-terminal for it - ensures sigint passed as normal so service unregistered properly
ssh -t root@192.168.0.252 "/mnt/us/dev/lifeguard"