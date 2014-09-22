V= 																	# verbose ON/OFF (default: OFF)
QEMU=qemu-system-i386
INFO="  [info] "
RUSTC:=rustc
OBJDIR=out
AS=as
SHELL := /bin/bash
LD=ld
QEMU_FLAGS=-serial mon:stdio

OBJS := $(addprefix $(OBJDIR)/,kernel.o boot.o, kernel.bin, disk.img)

all: $(OBJS)

$(OBJS): |$(OBJDIR)

$(OBJDIR)/kernel.o:
	$(RUSTC) --crate-type lib -o $@ --emit obj src/kernel.rs


$(OBJDIR):
	$Vmkdir -p $(OBJDIR)

$(OBJDIR)/boot.o: boot/boot.s | $(OBJDIR)
	$V$(AS) --32 $< -o $@

$(OBJDIR)/kernel.bin: boot/boot.ld $(OBJDIR)/kernel.o $(OBJDIR)/boot.o
	$V$(LD) -o $@ -T $^

$(OBJDIR)/disk.img: $(OBJDIR)/kernel.bin 
	dd if=/dev/zero of=$@ bs=512 count=8 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

clean:
	$Vrm -rf $(OBJDIR)

run: $(OBJDIR)/disk.img | $(OBJDIR)
	$Vecho "$(INFO)Running qemu simulation"
	$V$(QEMU) $(QEMU_FLAGS) -fda $^

debug: $(OBJDIR)/disk.img | $(OBJDIR)
	$Vecho "$(INFO)Running qemu in debug mode"
	$V$(QEMU) -s -S $(QEMU_FLAGS) -fda $^

.PHONY: clean run debug
