pub fn Data_EuclideanRing_intDegree(mut a0: i64) -> i64 {
    let abs_x = a0.abs();
    std::cmp::min(abs_x, 2147483647)
}

pub fn Data_EuclideanRing_intDiv(mut x: i64, mut y: i64) -> i64 {
    if y == 0 {
        return 0;
    }
    if y > 0 {
        x.div_euclid(y)
    } else {
        -x.div_euclid(-y)
    }
}

pub fn Data_EuclideanRing_intMod(mut x: i64, mut y: i64) -> i64 {
    if y == 0 {
        return 0;
    }
    let yy = y.abs();
    ((x % yy) + yy) % yy
}

pub fn Data_EuclideanRing_numDiv(mut x: f64, mut y: f64) -> f64 {
    x / y
}
