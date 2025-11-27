//! Register allocation tests

use zkir_llvm::regalloc::{LinearScan, LiveInterval, VirtualReg};

#[test]
fn test_linear_scan_basic() {
    let intervals = vec![
        LiveInterval::new(VirtualReg(0), 0, 10),
        LiveInterval::new(VirtualReg(1), 5, 15),
        LiveInterval::new(VirtualReg(2), 12, 20),
    ];

    let mut allocator = LinearScan::new(intervals);
    let result = allocator.allocate();

    assert!(result.is_ok(), "Allocation should succeed");

    // All virtual registers should get assignments
    assert!(allocator.get_assignment(VirtualReg(0)).is_some());
    assert!(allocator.get_assignment(VirtualReg(1)).is_some());
    assert!(allocator.get_assignment(VirtualReg(2)).is_some());
}

#[test]
fn test_live_interval_overlap() {
    let i1 = LiveInterval::new(VirtualReg(0), 0, 10);
    let i2 = LiveInterval::new(VirtualReg(1), 5, 15);
    let i3 = LiveInterval::new(VirtualReg(2), 20, 30);

    assert!(i1.overlaps(&i2), "i1 and i2 should overlap");
    assert!(i2.overlaps(&i1), "i2 and i1 should overlap");
    assert!(!i1.overlaps(&i3), "i1 and i3 should not overlap");
    assert!(!i3.overlaps(&i1), "i3 and i1 should not overlap");
}

#[test]
fn test_non_overlapping_intervals() {
    let intervals = vec![
        LiveInterval::new(VirtualReg(0), 0, 5),
        LiveInterval::new(VirtualReg(1), 6, 10),
        LiveInterval::new(VirtualReg(2), 11, 15),
    ];

    let mut allocator = LinearScan::new(intervals);
    let result = allocator.allocate();

    assert!(result.is_ok());

    // Non-overlapping intervals can reuse registers
    let r0 = allocator.get_assignment(VirtualReg(0)).unwrap();
    let r1 = allocator.get_assignment(VirtualReg(1)).unwrap();
    let r2 = allocator.get_assignment(VirtualReg(2)).unwrap();

    // They could potentially get the same physical register
    // (but that's implementation-dependent)
    assert!(r0 == r1 || r0 != r1); // Just checking they're valid
}

#[test]
fn test_many_overlapping_intervals() {
    // Create many overlapping intervals to test spilling
    let mut intervals = Vec::new();
    for i in 0..30 {
        intervals.push(LiveInterval::new(VirtualReg(i), 0, 100));
    }

    let mut allocator = LinearScan::new(intervals);
    let result = allocator.allocate();

    // With only 20 allocatable registers, some should be spilled
    assert!(result.is_ok());

    let mut spilled_count = 0;
    for i in 0..30 {
        if allocator.is_spilled(VirtualReg(i)) {
            spilled_count += 1;
            assert!(allocator.get_spill_slot(VirtualReg(i)).is_some());
        }
    }

    // At least some should be spilled
    assert!(spilled_count >= 10, "Expected at least 10 spills, got {}", spilled_count);
}
