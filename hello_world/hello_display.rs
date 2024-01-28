use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point3D {
    x:f64,
    y:f64,
    z:f64
}

impl fmt::Display for Point3D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z:{}", self.x, self.y, self.z)
    }
}

fn main() {
    let min_max = MinMax(15,20);
    
    println!("Display min max: {}", min_max);
    println!("Debug min max: {:?}", min_max);

    let bigger_range_size = MinMax(-600,600);
    let smaller_range_size = MinMax(-6,6);

    println!("Big range is {big} and small is {small}", 
             small =smaller_range_size,
             big = bigger_range_size
    );

    let point3_d = Point3D{ x:6.78, y:9.87, z:0.01 };
    println!("Compare 3D points:");
    println!("Display min max: {}", point3_d);
    println!("Debug min max: {:?}", point3_d);
}

