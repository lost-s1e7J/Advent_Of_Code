fn main() {
    let data = include_str!("input").lines();
    let mut numbers_int = Vec::new();
    for calibration in data {
        let mut chars = calibration.chars();
        let mut vector = Vec::new();
        for _ in 0..calibration.len() {
            let value = chars.next().unwrap();
            if value.is_numeric() {
                vector.push(value);
            }
        }
        let mut number = String::new();
        number.push(vector[0]);
        number.push(vector[vector.len() - 1]);
        numbers_int.push(number.parse::<i32>().unwrap());
    }
    let resultado = numbers_int.iter().sum::<i32>();
    println!("{resultado:?}");
}
