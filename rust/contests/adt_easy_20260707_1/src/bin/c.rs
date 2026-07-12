use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }
    let theta: f64 = d / 180. * PI;
    let x: f64 = a * theta.cos() - b * theta.sin();
    let y: f64 = b * theta.cos() + a * theta.sin();
    println!("{} {}", x, y);
}
