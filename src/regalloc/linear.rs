//! Linear scan register allocation

use super::{LiveInterval, PhysicalReg, RegAllocError, RegAllocResult, VirtualReg};
use std::collections::HashMap;
use zkir_spec::Register;

pub struct LinearScan {
    /// Live intervals for each virtual register
    intervals: Vec<LiveInterval>,

    /// Physical register assignments
    assignments: HashMap<VirtualReg, PhysicalReg>,

    /// Spill slots for spilled registers
    spill_slots: HashMap<VirtualReg, i32>,

    /// Next spill slot offset
    next_spill: i32,
}

impl LinearScan {
    pub fn new(intervals: Vec<LiveInterval>) -> Self {
        Self {
            intervals,
            assignments: HashMap::new(),
            spill_slots: HashMap::new(),
            next_spill: 0,
        }
    }

    /// Run linear scan allocation
    pub fn allocate(&mut self) -> RegAllocResult<()> {
        // Sort intervals by start point
        self.intervals.sort_by_key(|i| i.start);

        let mut active: Vec<LiveInterval> = Vec::new();
        let mut free_regs = self.get_allocatable_regs();

        for interval in self.intervals.clone() {
            // Expire old intervals
            self.expire_old(&mut active, &mut free_regs, interval.start);

            if free_regs.is_empty() {
                // Need to spill
                self.spill_at_interval(&mut active, &interval)?;
            } else {
                // Allocate register
                let reg = free_regs.pop().unwrap();
                self.assignments.insert(interval.vreg, reg);
                active.push(interval.clone());
                active.sort_by_key(|i| i.end);
            }
        }

        Ok(())
    }

    /// Get allocatable physical registers
    fn get_allocatable_regs(&self) -> Vec<PhysicalReg> {
        let mut regs = Vec::new();

        // t0-t7 (r8-r15)
        for i in 8..16 {
            regs.push(Register::from_index(i).unwrap());
        }

        // s0-s7 (r16-r23)
        for i in 16..24 {
            regs.push(Register::from_index(i).unwrap());
        }

        // t8-t11 (r24-r27)
        for i in 24..28 {
            regs.push(Register::from_index(i).unwrap());
        }

        regs
    }

    /// Expire old intervals and free their registers
    fn expire_old(
        &mut self,
        active: &mut Vec<LiveInterval>,
        free_regs: &mut Vec<PhysicalReg>,
        current: u32,
    ) {
        active.retain(|interval| {
            if interval.end <= current {
                // This interval is done, free its register
                if let Some(reg) = self.assignments.get(&interval.vreg) {
                    free_regs.push(*reg);
                }
                false // Remove from active
            } else {
                true // Keep in active
            }
        });
    }

    /// Spill at interval
    fn spill_at_interval(
        &mut self,
        active: &mut [LiveInterval],
        interval: &LiveInterval,
    ) -> RegAllocResult<()> {
        // Find interval with latest end point
        let spill = active
            .iter()
            .max_by_key(|i| i.end)
            .ok_or(RegAllocError::OutOfRegisters)?;

        if spill.end > interval.end {
            // Spill the interval with the latest end
            let reg = self
                .assignments
                .get(&spill.vreg)
                .ok_or(RegAllocError::OutOfRegisters)?;

            // Assign register to current interval
            self.assignments.insert(interval.vreg, *reg);

            // Allocate spill slot for spilled interval
            let slot = self.next_spill;
            self.next_spill -= 4;
            self.spill_slots.insert(spill.vreg, slot);

            // Update active list
            // Remove spill, add current
            // (This is simplified - real implementation would need proper updating)
        } else {
            // Spill current interval
            let slot = self.next_spill;
            self.next_spill -= 4;
            self.spill_slots.insert(interval.vreg, slot);
        }

        Ok(())
    }

    /// Get the assignment for a virtual register
    pub fn get_assignment(&self, vreg: VirtualReg) -> Option<PhysicalReg> {
        self.assignments.get(&vreg).copied()
    }

    /// Check if a virtual register was spilled
    pub fn is_spilled(&self, vreg: VirtualReg) -> bool {
        self.spill_slots.contains_key(&vreg)
    }

    /// Get spill slot for a virtual register
    pub fn get_spill_slot(&self, vreg: VirtualReg) -> Option<i32> {
        self.spill_slots.get(&vreg).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scan() {
        let intervals = vec![
            LiveInterval::new(VirtualReg(0), 0, 10),
            LiveInterval::new(VirtualReg(1), 5, 15),
            LiveInterval::new(VirtualReg(2), 12, 20),
        ];

        let mut allocator = LinearScan::new(intervals);
        allocator.allocate().unwrap();

        assert!(allocator.get_assignment(VirtualReg(0)).is_some());
        assert!(allocator.get_assignment(VirtualReg(1)).is_some());
        assert!(allocator.get_assignment(VirtualReg(2)).is_some());
    }
}
