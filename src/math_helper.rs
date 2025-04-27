pub fn add(values:(i64,i64)) ->i64{
    values.0+values.1
}
pub fn subtract(values:(i64,i64)) ->i64{
    values.0-values.1
}
pub fn multiply(values:(i64,i64)) ->i64{
    let values_bigger:(i64,i64) = (values.0 as i64, values.1 as i64);
    values_bigger.0 * values_bigger.1
}
pub fn divide(values:(i64,i64)) ->f64{
    if values.1 == 0 {
        println!("Divide by 0 not allowed!");
        return -1f64;
    }
    values.0 as f64 / values.1 as f64
}