// https://leetcode.com/problems/design-parking-system/description/
struct ParkingSystem {
    slots: Vec<i32>,
}
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slots: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[car_type as usize - 1] > 0 {
            self.slots[car_type as usize - 1] -= 1;
            true
        } else {
            false
        }
    }
}

