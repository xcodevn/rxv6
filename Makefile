QEMU=qemu-system-i386
INFO=" [info] "
RUSTC := rustc
OBJDIR := out
AS=as
SHELL := /bin/bash
LD := ld
QEMU_FLAGS := -serial mon:stdio
CC := gcc
V=0

TOP = .

OBJS := $(addprefix $(OBJDIR)/,kernel.o boot.o readline.o printfmt.o string.o printf.o console.o)

GCC_LIB := $(shell $(CC) $(CFLAGS) -print-libgcc-file-name)

# Run 'make V=1' to turn on verbose commands, or 'make V=0' to turn them off.
ifeq ($(V),1)
override V =
endif
ifeq ($(V),0)
override V = @
endif

all: compile run
	$Vecho "> compile and run"

compile: $(OBJS)
	$Vecho "> compile all objs"


$(OBJDIR)/kernel.o: src/kernel.rs | $(OBJDIR)
	$Vecho + rustc $< -o $@
	$V$(RUSTC) -Z no-landing-pads -g --crate-type lib -o $@ --emit obj src/kernel.rs

$(OBJDIR):
	$Vecho + mkdir $(OBJDIR)
	$Vmkdir -p $(OBJDIR)

$(OBJDIR)/boot.o: boot/boot.s | $(OBJDIR)
	$Vecho + as boot.s
	$V$(AS) --32 $< -o $@

$(OBJDIR)/kernel.elf: boot/boot.ld $(OBJS)
	$Vecho + ld kernel.elf
	$V$(LD) -g -o $@ -T $^ $(GCC_LIB)

$(OBJDIR)/kernel.bin: $(OBJDIR)/kernel.elf
	$Vecho "+ objcopy kernel.elf to binary format (kernel.bin)"
	$Vobjcopy -O binary $(OBJDIR)/kernel.elf $(OBJDIR)/kernel.bin

$(OBJDIR)/disk.img: $(OBJDIR)/kernel.bin 
	$Vecho + create disk.img with size 24 x 512 bytes
	$Vdd if=/dev/zero of=$@ bs=512 count=24 &>/dev/null
	$Vecho + overrided by kernel.bin
	$Vcat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

$(OBJDIR)/%.o: libc/%.c
	$Vecho + cc $< -o $@
	$V$(CC) -ggdb -fno-omit-frame-pointer -Wall -Wno-format -Wno-unused -Werror -gstabs -m32 -O1 -fno-builtin -I$(TOP)/libc -c -o $@ $<

clean:
	$Vecho + rm all objs
	$Vrm -rf $(OBJDIR)

run: $(OBJDIR)/disk.img $(OBJS)| $(OBJDIR)
	$Vecho "> Running qemu simulation"
	$V$(QEMU) $(QEMU_FLAGS) -fda $<

debug: $(OBJDIR)/disk.img | $(OBJDIR)
	$Vecho "> Running qemu in debug mode"
	$V$(QEMU) -s -S $(QEMU_FLAGS) -fda $<

.PHONY: all clean run debug

