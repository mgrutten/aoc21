use std::collections::HashSet;
use std::error::Error;


/*

fn sign(num: i32) -> i32 {
    match num.cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}


 */

fn main() -> Result<(), Box<dyn Error>> {

    // target area: x=20..30, y=-10..-5
    // target area: x=135..155, y=-102..-78

    let mut velocities = HashSet::new();
    for k in 0..210 {
        let mut y_vel = Vec::new();
        //for yp in -10..-4 {
        for yp in -102..-77 {
            let yv0 = yp as f64 / k as f64 + (k as f64 - 1.0) / 2.0;

            if (yv0 - yv0.round()).abs() < 1e-6 {
                y_vel.push(yv0 as i64);
            }
        }

        let mut x_vel = Vec::new();
        //for xp in 20..31 {
        for xp in 135..156 {
            // When xv > k (i.e. min(xv, k) == k)
            let xv01 = xp as f64 / k as f64 + (k as f64 - 1.0) / 2.0;
            // When xv < k (i.e. min(xv, k) == xv)
            let xv02 = -0.5 + 0.5 * (1.0 + 8.0 * xp as f64).sqrt();

            if xv01 >= k as f64 && (xv01 - xv01.round()).abs() < 1e-6 {
                x_vel.push(xv01 as i64);
            } else if xv02 <= k as f64 && (xv02 - xv02.round()).abs() < 1e-6 {
                x_vel.push(xv02 as i64);
            }
        }

        for xv in x_vel {
            for yv in y_vel.iter() {
                velocities.insert((xv, *yv));
            }
        }

        println!("{} count: {}", k, velocities.len());
    }


    println!("count: {}", velocities.len());

    /*
let yv0 = 101;
let mut yp = 0;
let mut yv = yv0;
for k in 1..205 {
    yp = yp + yv;
    yv = yv - 1;

    println!("yp: {}", yp);
    println!("yv: {}", yv);

    println!("yp: {}", k * yv0 - k * (k - 1) / 2);
    println!("yv: {}", yv0 - k);
    println!();
}

println!("{}", 101 * 102 / 2);

 */

    /*
    let xv0 = -6;
    let mut xp = 0;
    let mut xv = xv0;
    for k in 1..10 {
        xp = xp + xv;
        xv = xv - sign(xv);

        println!("xp: {}", xp);
        println!("xv: {}", xv);

        let max_k = min(k, xv0.abs());
        println!("xp: {}", max_k * xv0 - sign(xv0) * max_k * (max_k - 1) / 2);
        println!("xv: {}", xv0 - sign(xv0) * max_k);
        println!();
    }


     */


    Ok(())
}


