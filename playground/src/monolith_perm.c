#include "monolith_perm.h"

MappedMonolith map_monolith() {
    MappedMonolith ret_map = {
        .base = MAP_FAILED,
        .dev_mem_fd = -1
    };
    void *map_base = MAP_FAILED; // Initialize map_base to the error value

    ret_map.dev_mem_fd = open("/dev/mem", O_RDWR | O_SYNC);
    if (ret_map.dev_mem_fd == -1) {
        perror("Error opening /dev/mem (root privileges?)");
    }

    map_base = mmap(0, MAP_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, ret_map.dev_mem_fd, HW_BASE_ADDRESS);
    if (map_base == MAP_FAILED) {
        perror("Error mapping memory via mmap");
        close(ret_map.dev_mem_fd); 
    }

    ret_map.base = (volatile uint32_t*)map_base;

    return ret_map;
}

void unmap_monolith(MappedMonolith *map) {
    // Unmap memory
    if (munmap((void*)map->base, MAP_SIZE) == -1) {
        perror("Error unmapping memory");
        // Continue to close fd anyway
    }
    map->base = MAP_FAILED;

    // Close file descriptor
    if (close(map->dev_mem_fd) == -1) {
        perror("Error closing /dev/mem file descriptor");
    }
    map->dev_mem_fd = -1;
}

uint32_t monolith_hash(MappedMonolith *acc, uint32_t input) {
    acc->base[0] = (input << 1) | 1;

    // Example: For a million computations, about 20-30 calls to this function will return the value from the prev call without this fine tuning code.
    // Below two lines should force flush previous write. 
    volatile uint32_t tmp1 = acc->base[0];
    volatile uint32_t tmp2 = acc->base[2]; 
    
    // Sleep here as to let writes propagate through the OS to the hardware.
    // The peripheral will set the valid flag to 0 when the value changes, earlier reads to the output will read the previous output (BAD!).
    // AXI bus to peripheral runs at ~47MHz (21ns period).
    // CPU runs at 650MHz (~1.53ns period).
    // Linux sleep is at least a few microseconds due to scheduler.
    // Tight loop sleep this since this function will take less than the 100ms sched quanta, so we shouldn't be switched out.
    for (volatile int i = 0; i < 50; i++) {
        asm("MOV r0, r0"); // NOP on ARMv7
    }

    while(1) {
        volatile uint32_t read_value = acc->base[2];
        volatile uint32_t output = read_value >> 1;
        volatile uint32_t valid = read_value & 1;

		if (valid) return output;
	}
};

uint32_t monolith_compress(MappedMonolith *acc, uint32_t input1, uint32_t input2) {
    acc->base[1] = (input2 << 1) | 1;
    acc->base[0] = (input1 << 1) | 1;

    // Example: For a million computations, about 20-30 calls to this function will return the value from the prev call without this fine tuning code.
    // Below two lines should force flush previous write. 
    volatile uint32_t tmp2 = acc->base[1];
    volatile uint32_t tmp1 = acc->base[0];
    volatile uint32_t tmp3 = acc->base[2]; 
    
    // Sleep here as to let writes propagate through the OS to the hardware.
    // The peripheral will set the valid flag to 0 when the value changes, earlier reads to the output will read the previous output (BAD!).
    // AXI bus to peripheral runs at ~47MHz (21ns period).
    // CPU runs at 650MHz (~1.53ns period).
    // Linux sleep is at least a few microseconds due to scheduler.
    // Tight loop sleep this since this function will take less than the 100ms sched quanta, so we shouldn't be switched out.
    for (volatile int i = 0; i < 50; i++) {
        asm("MOV r0, r0"); // NOP on ARMv7
    }

    while(1) {
        volatile uint32_t read_value = acc->base[2];
        volatile uint32_t output = read_value >> 1;
        volatile uint32_t valid = read_value & 1;

		if (valid) return output;
	}
};