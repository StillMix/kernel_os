cargo clean  
cargo +nightly bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-kernel_os.bin
