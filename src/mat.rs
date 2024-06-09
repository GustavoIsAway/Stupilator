pub fn soma(a: f64, b: f64) -> f64{
  a + b
}

pub fn sub(a: f64, b: f64) -> f64{
  a - b
}  

pub fn mult(a: f64, b: f64) -> f64{
  a * b
} 

pub fn div(a: f64, b: f64) -> Result<f64, &'static str>{
  if b == 0.0{
    Err("undefined")
  }
  else{
    Ok(a / b)
  }
}  