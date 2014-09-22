QEMU=qemu-system-i386
INFO="  [info] "
RUSTC := rustc
OBJDIR := out
AS=as
SHELL := /bin/bash
LD := ld
QEMU_FLAGS := -serial mon:stdio
CC := gcc

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

all: $(OBJS)
	echo "Nothing"


$(OBJDIR)/kernel.o: src/kernel.rs | $(OBJDIR)
	$(RUSTC) -g --crate-type lib -o $@ --emit obj src/kernel.rs

$(OBJDIR):
	$Vmkdir -p $(OBJDIR)

$(OBJDIR)/boot.o: boot/boot.s | $(OBJDIR)
	$V$(AS) --32 $< -o $@

$(OBJDIR)/kernel.elf: boot/boot.ld $(OBJS)
	$V$(LD) -g -o $@ -T $^ $(GCC_LIB)

$(OBJDIR)/kernel.bin: $(OBJDIR)/kernel.elf
	objcopy -O binary $(OBJDIR)/kernel.elf $(OBJDIR)/kernel.bin

$(OBJDIR)/disk.img: $(OBJDIR)/kernel.bin 
	dd if=/dev/zero of=$@ bs=512 count=24 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

$(OBJDIR)/%.o: lib/%.c
	$V$(CC) -ggdb -fno-omit-frame-pointer -Wall -Wno-format -Wno-unused -Werror -gstabs -m32 -O1 -fno-builtin -I$(TOP) -c -o $@ $<

clean:
	$Vrm -rf $(OBJDIR)

run: $(OBJDIR)/disk.img | $(OBJDIR)
	$Vecho $(OBJS)
	$Vecho "$(INFO)Running qemu simulation"
	$V$(QEMU) $(QEMU_FLAGS) -fda $<

debug: $(OBJDIR)/disk.img | $(OBJDIR)
	$Vecho "$(INFO)Running qemu in debug mode"
	$V$(QEMU) -s -S $(QEMU_FLAGS) -fda $<

.PHONY: all clean run debug

