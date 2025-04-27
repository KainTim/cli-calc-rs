pub fn add(values:(f64,f64)) ->f64{
    values.0+values.1
}
pub fn subtract(values:(f64,f64)) ->f64{
    values.0-values.1
}
pub fn multiply(values:(f64,f64)) ->f64{
    values.0 * values.1
}
pub fn divide(values:(f64,f64)) ->f64{
    if values.1 == 0f64 {
        println!("Divide by 0 not allowed!");
        return -1f64;
    }
    values.0 / values.1
}