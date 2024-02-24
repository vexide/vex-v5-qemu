qemu-system-arm \
    -machine none \
    -cpu cortex-a7 \
    -m 256M \
    -device loader,file=test.bin,force-raw=on,addr=0x03800000,cpu-num=0 \
    -device loader,file=sdk-shim/shim.bin,force-raw=on,addr=0x037ec000 \
    -S \
    -s \
    -nographic