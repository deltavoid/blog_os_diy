

image := target/x86_64-blog_os/debug/bootimage-blog_os_diy.bin

.PHONY : run target clean
run : target
	qemu-system-x86_64 \
    -drive format=raw,file=$(image) \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
    
target :
	bootimage build --target x86_64-blog_os.json


.PHONY : test test-basic-boot test-exception-breakpoint test-exception-simple-breakpoint test-exception-double-fault-stack-overflow
test : test-exception-double-fault-stack-overflow


test-basic-boot :
	bootimage build --bin $@ --target x86_64-blog_os.json
	qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-$@.bin \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none

test-exception-breakpoint :
	bootimage build --bin $@ --target x86_64-blog_os.json
	qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-$@.bin \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none

test-exception-double-fault-stack-overflow :
	bootimage build --bin $@ --target x86_64-blog_os.json
	qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-$@.bin \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
#    -display none

test-exception-simple-breakpoint :
	bootimage build --bin $@ --target x86_64-blog_os.json
	qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-$@.bin \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
#    -display none

clean : 
	cargo clean