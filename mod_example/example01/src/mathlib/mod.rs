pub mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 {
        f.sin()
    }
    pub fn cos(f: f64) -> f64 {
        f.cos()
    }
    pub fn tan(f: f64) -> f64 {
        f.tan()
    }
    pub fn add(a: Complex, b: Complex) -> Complex {
        (a.0 + b.0, a.1 + b.1)
    }
}