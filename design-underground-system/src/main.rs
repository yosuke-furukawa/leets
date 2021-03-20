use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    checkin_map: HashMap<i32, (String, i32)>,
    sum_map: HashMap<(String, String), (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkin_map.entry(id).or_insert((station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start_time)) = self.checkin_map.remove(&id) {
            let (sum, num) = self
                .sum_map
                .entry((start_station, station_name))
                .or_insert((0, 0));
            *sum += t - start_time;
            *num += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some((sum, num)) = self.sum_map.get(&(start_station, end_station)) {
            *sum as f64 / *num as f64
        } else {
            0.0
        }
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

fn main() {
    let mut obj = UndergroundSystem::new();
    obj.check_in(1, "Tokyo".to_string(), 1);
    obj.check_out(1, "Kyoto".to_string(), 10);
    let ret_3: f64 = obj.get_average_time("Tokyo".to_string(), "Kyoto".to_string());
    println!("{}", ret_3);
}
