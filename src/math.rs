pub fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() 
}