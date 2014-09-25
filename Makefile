QEMU=qemu-system-i386
INFO=" [info] "
RUSTC := LD_LIBRARY_PATH=/usr/local/lib rustc
OBJDIR := obj
AS=as -32
SHELL := /bin/bash
LD := ld -m elf_i386
QEMU_FLAGS := -serial mon:stdio
CC_FLAGS := -nostdinc -fno-omit-frame-pointer -Wall -Wno-format -Wno-unused -Werror -gstabs -O1 -fno-builtin
CC := gcc -pipe -m32
V=0

TOP = .

OBJS := $(addprefix $(OBJDIR)/,entry.o entrypgdir.o init.o readline.o printfmt.o string.o printf.o console.o libm.o libkernel.a)

GCC_LIB := $(shell $(CC) $(CFLAGS) -print-libgcc-file-name)

# Run 'make V=1' to turn on verbose commands, or 'make V=0' to turn them off.
ifeq ($(V),1)
override V =
endif
ifeq ($(V),0)
override V = @
endif

all: compile run

$(OBJDIR):
	$Vecho  mkdir $(OBJDIR)
	$Vmkdir -p $(OBJDIR)


compile: $(OBJS) | $(OBJDIR)
	$Vecho "> compile all objs"

$(OBJDIR)/%.o: lib/%.rs
	$Vecho  rc $< -o $@
	$V$(RUSTC) -g -o $@ --emit=obj $^


$(OBJDIR)/libkernel.a: src/kernel.rs | $(OBJDIR)
	$Vecho  rc $< -o $@
	$V$(RUSTC) --dep-info $(OBJDIR)/.deps -g -o $@ src/kernel.rs

$(OBJDIR)/boot0: boot/boot.ld boot/boot.S boot/main.c | $(OBJDIR)
	$Vecho  as boot.s
	$V$(CC) -E boot/boot.S  -I$(TOP)/libc > $(OBJDIR)/boot.s
	$V$(AS) --32 $(OBJDIR)/boot.s -o $(OBJDIR)/bootA.o
	$Vecho  cc boot/main.c
	$V$(CC) -c -o $(OBJDIR)/bootB.o boot/main.c $(CC_FLAGS) -I$(TOP)/libc
	$Vecho  ld bootA.o bootB.o -o boot0
	$V$(LD) -g -o $@ -T boot/boot.ld $(OBJDIR)/bootA.o $(OBJDIR)/bootB.o

$(OBJDIR)/%.o: src/%.S | $(OBJDIR)
	$Vecho cc $^
	$V$(CC) -c -o $@ $^ -I$(TOP)/libc $(CC_FLAGS)

$(OBJDIR)/%.o: src/%.c | $(OBJDIR)
	$Vecho cc $^
	$V$(CC) -c -o $@ $^ -I$(TOP)/libc $(CC_FLAGS)

$(OBJDIR)/kernel.elf: src/kernel.ld $(OBJS)
	$Vecho  ld kernel.elf
	$V$(LD) -g -o $@ -T $^ -nostdlib --unresolved-symbols=ignore-all			# use CC as our linker (easier with math lib)

$(OBJDIR)/kernel.bin: $(OBJDIR)/kernel.elf
	$Vecho "objcopy kernel.elf to binary format (kernel.bin)"
	$Vobjcopy -O binary $(OBJDIR)/kernel.elf $(OBJDIR)/kernel.bin

$(OBJDIR)/disk.img: $(OBJDIR)/boot0 $(OBJDIR)/kernel.elf
	$Vecho  create disk.img with size 4 x 1M
	$Vdd if=/dev/zero of=$@ bs=1M count=4 &>/dev/null
	$Vecho  overrided by $^
	$Vcat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

$(OBJDIR)/%.o: libc/%.c | $(OBJDIR)
	$Vecho  cc $< -o $@
	$V$(CC) $(CC_FLAGS) -I$(TOP)/libc -c -o $@ $<

clean:
	$Vecho  rm all objs
	$Vrm -rf $(OBJDIR)

run: $(OBJDIR)/disk.img $(OBJS) 
	$Vecho "> Running qemu simulation"
	$V$(QEMU) $(QEMU_FLAGS) -hda $< 2>/dev/null

debug: $(OBJDIR)/disk.img $(OBJS)
	$Vecho "> Running qemu in debug mode"
	$V$(QEMU) -s -S $(QEMU_FLAGS) -hda $< 2>/dev/null



-include $(OBJDIR)/.deps

.PHONY: all clean run debug

