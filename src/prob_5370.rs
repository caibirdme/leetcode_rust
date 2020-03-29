use std::collections::HashMap;

struct UndergroundSystem {
    people: HashMap<i32, (String, i32)>,
    stations: HashMap<String, HashMap<String, (i64, i64)>>
}

impl UndergroundSystem {

    fn new() -> Self {
        Self {
            people: HashMap::new(),
            stations: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.people.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start)) = self.people.get(&id) {
            let (times, total) = self.stations.entry(start_station.clone())
                .or_insert(HashMap::new())
                .entry(station_name).or_insert((0, 0));
            *times += 1;
            *total += (t - *start) as i64;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(destinations) = self.stations.get(&start_station) {
            if let Some(&(times, total)) = destinations.get(&end_station) {
                return total as f64/(times as f64);
            }
        }
        0f64
    }
}