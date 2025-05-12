#ifndef MONOLITH_PERM
#define MONOLITH_PERM

#include <stdio.h>
#include <unistd.h>
#include <stdint.h>
#include <sys/mman.h>
#include <fcntl.h>

#define MAP_SIZE 16
#define HW_BASE_ADDRESS 0x43C00000

/**
    Contains various data about the accelerator such as mapped base address, or the descriptor used for /dev/mem.
 */
typedef struct MappedMonolith_s {
    int dev_mem_fd; // File descriptor for /dev/mem used in mmap.
    volatile uint32_t *base; // Pointer to mapped virtual address of Monolith Hardware accelerator.
} MappedMonolith;

/**
    Maps the physical address of the accelerator into the virtual address space of the process.
 */
MappedMonolith map_monolith();

/**
    Unmaps the address of the accelerator from the process' virtual address space.
 */
void unmap_monolith(MappedMonolith *map);

uint32_t monolith_hash(MappedMonolith acc, uint32_t input);
uint32_t monolith_compress(MappedMonolith acc, uint32_t input1, uint32_t input2);

#endif