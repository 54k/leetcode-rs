// https://leetcode.com/problems/snapshot-array/description/
use std::collections::BTreeMap;

struct SnapshotArray {
    snaps: Vec<BTreeMap<i32, i32>>,
    id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut snaps = vec![BTreeMap::new(); length as usize];
        for m in &mut snaps {
            m.insert(0, 0);
        }
        Self { snaps, id: 0 }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snaps[index as usize].insert(self.id, val);
    }

    fn snap(&mut self) -> i32 {
        self.id += 1;
        self.id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        *self.snaps[index as usize]
            .range(0..=snap_id)
            .last()
            .unwrap()
            .1
    }
}
