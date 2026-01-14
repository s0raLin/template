use anyhow::Result;

pub fn add(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

pub fn subtract(a: i32, b: i32) -> Result<i32> {
    Ok(a - b)
}

pub fn multiply(a: i32, b: i32) -> Result<i32> {
    Ok(a * b)
}

pub fn divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
        anyhow::bail!("除数不能为零");
    }
    Ok(a / b)
}
