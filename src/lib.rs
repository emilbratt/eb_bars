#![allow(unused)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [f64; 24] = [
        22.5,  5.2, 11.5,  1.7, 10.5,  0.5, 1.5,  1.0,
         2.9,  5.2,  3.1,  6.4,  3.1, 15.5, 5.1,  2.5,
        12.5, 16.0,  7.1, 13.8,  6.7,  8.1, 6.4, 21.1,
    ];

    #[test]
    fn positive_values() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let values = VALUES;
        println!("{:?}", values);
    }

    #[test]
    fn negative_values() {
        let mut values: Vec<f64> = Vec::with_capacity(24);
        for i in 0..VALUES.len() {
            if i % 3 == 1 || i % 5 == 1 {
                values.push(VALUES[i] as f64 * -1.0);
            } else {
                values.push(VALUES[i]);
            }
        }

        println!("{:?}", values);
    }

    #[test]
    fn positive_values_large() {
        let values: Vec<f64> = VALUES.iter().map(|v| v.abs().sqrt() * 10000000.0).collect();
        println!("{:?}", values);
    }

    #[test]
    fn negative_values_values_large() {
        let mut values: Vec<f64> = Vec::with_capacity(24);
        for i in 0..VALUES.len() {
            let v =  if i % 3 == 1 || i % 5 == 1 {
                (VALUES[i] as f64).sqrt() * -1.0
            } else {
                VALUES[i].sqrt()
            };
            values.push(v * 10000000.0);
        }

        println!("{:?}", values);
    }

    #[test]
    fn with_x_markers() {
        let values = VALUES;

        let mut markers: Vec<f64> = Vec::with_capacity(24);
        for i in 0..VALUES.len() {
            markers.push(i as f64);
        }

        println!("{:?}", markers);
    }

    #[test]
    fn with_less_x_markers() {
        let values = VALUES;

        let mut markers: Vec<f64> = Vec::with_capacity(24);
        for i in 0..VALUES.len() {
            if i % 3 == 0 {
                markers.push(i as f64);
            }
        }

        println!("{:?}", markers);
    }

}
