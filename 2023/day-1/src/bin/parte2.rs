fn main() {
    let data = include_str!("input").lines();
    let number_text = ["one","two","three","four","five","six","seven","eight","nine"];
    let number_int = ["o1e","t2o","t3e","f4r","f5e","s6x","s7n","e8t","n9e"];
    let mut numbers_int = Vec::new();
    for calibration in data {
        let mut clean_calibration = calibration.to_string();
        for i in 0..9 {
            clean_calibration = clean_calibration.replace(number_text[i], number_int[i]);
        }
        let mut chars = clean_calibration.chars();
        let mut vector = Vec::new();
        for _ in 0..clean_calibration.len() {
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
