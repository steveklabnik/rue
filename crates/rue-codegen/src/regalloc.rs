use crate::{Register, VReg};
use std::collections::HashMap;

/// Simple linear scan register allocator
pub struct RegisterAllocator {
    /// Mapping from virtual registers to physical registers
    allocation: HashMap<VReg, Register>,
    /// Available physical registers (in order of preference)
    available_registers: Vec<Register>,
    /// Next register to allocate
    next_register_index: usize,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            allocation: HashMap::new(),
            // Use available x86-64 registers
            // Reserve rax for return values, rsp/rbp for stack
            available_registers: vec![
                Register::Rbx,
                Register::Rcx,
                Register::Rdx,
                Register::Rsi,
                Register::Rdi,
            ],
            next_register_index: 0,
        }
    }

    /// Allocate a physical register for a virtual register
    pub fn allocate(&mut self, vreg: VReg) -> Register {
        if let Some(&physical_reg) = self.allocation.get(&vreg) {
            // Already allocated
            physical_reg
        } else {
            // Allocate next available register (simple round-robin)
            let physical_reg =
                self.available_registers[self.next_register_index % self.available_registers.len()];
            self.next_register_index += 1;

            self.allocation.insert(vreg, physical_reg);
            physical_reg
        }
    }

    /// Get the allocation mapping
    pub fn get_allocation(&self) -> &HashMap<VReg, Register> {
        &self.allocation
    }

    /// Get the physical register for a virtual register (must be already allocated)
    pub fn get_register(&self, vreg: VReg) -> Option<Register> {
        self.allocation.get(&vreg).copied()
    }
}

impl Default for RegisterAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_allocation() {
        let mut allocator = RegisterAllocator::new();

        let vreg1 = VReg(1);
        let vreg2 = VReg(2);

        let reg1 = allocator.allocate(vreg1);
        let reg2 = allocator.allocate(vreg2);

        // Should get consistent allocation
        assert_eq!(allocator.allocate(vreg1), reg1);
        assert_eq!(allocator.allocate(vreg2), reg2);

        // Should allocate different registers
        assert_ne!(reg1, reg2);
    }

    #[test]
    fn test_round_robin_allocation() {
        let mut allocator = RegisterAllocator::new();

        // Allocate more VRegs than available physical registers
        let mut vregs = Vec::new();
        let mut allocations = Vec::new();

        for i in 0..10 {
            let vreg = VReg(i);
            vregs.push(vreg);
            allocations.push(allocator.allocate(vreg));
        }

        // Should reuse registers in round-robin fashion
        assert_eq!(allocations[0], allocations[5]); // Wraparound after 5 registers
    }
}
