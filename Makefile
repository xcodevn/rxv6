
V= 																						# verbose ON/OFF (default: OFF)
QEMU=qemu-system-i386
INFO="  [info] "
RC=rustc																			# Rust compiler
OUTDIR=out
AS=as
SHELL := /bin/bash

all: 
	$Vecho "$(INFO)All we have now is nothing"

sim: $(OUTDIR)/kernel.bin
	$Vecho "$(INFO)Running qemu simulation"
	$V$(QEMU) $(QEMU_FLAGS) $^

mkoutdir:
	$Vmkdir -p $(OUTDIR)

$(OUTDIR)/kernel.bin: mkoutdir boot/boot.ld boot/boot.s
	$V$(AS) --32 boot/boot.s -o $(OUTDIR)/boot.o
	$Vld -T boot/boot.ld -o $@ $(OUTDIR)/boot.o

clean:
	$Vrm -rf $(OUTDIR)

