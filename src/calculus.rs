pub trait Derivative {
    fn derivative(&self) -> Self;

    fn nth_derivative(&self, n: usize) -> Self;
}

pub trait Integral {
    fn integral(&self) -> Self;
    
    fn nth_integral(&self, n: usize) -> Self;
}