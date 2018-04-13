use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point{x, y, z}
    }

    /// Calculate the distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)
         + (self.z - other.z).powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}\n", self.x)?;
        write!(f, "Y: {}\n", self.y)?;
        write!(f, "Z: {}", self.z)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_distance() {
        let (x, y, z) = (1_f64, 2_f64, 3_f64);
        let first = Point::new(x, y, z);
        let second = Point::new(x, y, z);
        assert_eq!(0_f64, first.distance(&second));
    }

    #[test]
    fn one_distance() {
        let (x, y, z) = (1_f64, 2_f64, 3_f64);
        let first = Point::new(x, y, z);
        let second = Point::new(x + 1_f64, y, z);
        assert_eq!(1_f64, first.distance(&second));
    }

    #[test]
    fn neg_pos_positive_distance() {
        let (x, y, z) = (1_f64, 2_f64, -3_f64);
        let first = Point::new(x, y, z);
        let second = Point::new(x + 1_f64, y, z);
        assert_eq!(1_f64, first.distance(&second));
    }

    #[test]
    fn three_four_five() {
        let (x, y, z) = (0_f64, 3_f64, 4_f64);
        let first = Point::new(x, y, z);
        let second = Point::new(0_f64, 0_f64, 0_f64);
        assert_eq!(5_f64, first.distance(&second));
    }

    #[test]
    fn all_zeroes() {
        let (x, y, z) = (0_f64, 0_f64, 0_f64);
        let first = Point::new(x, y, z);
        let second = Point::new(x, y, z);
        assert_eq!(0_f64, first.distance(&second));
    }
}
