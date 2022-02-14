fn main() {
    println!("{}", production_rate_per_hour(8));
    println!("{}", working_items_per_minute(8));
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut prod_rate: f64 =  match speed {
        1 | 2 | 3 | 4 => 1.0,
        5 | 6 | 7 | 8 =>  0.9,
        9 | 10 => 0.77,
        _ => 0.0,
    };
    let speed_f = speed as f64;
    prod_rate = 221.0 * prod_rate * speed_f;
    prod_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_rate = production_rate_per_hour(speed) as f64;
    let items_per_min = prod_rate / 60.0;
    let u_items_per_min = items_per_min as u32;
    u_items_per_min
}
