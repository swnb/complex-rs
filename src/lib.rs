use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Complex<T: Copy> {
    real: T,
    imaginary: T,
}

#[macro_export]
macro_rules! c {
    ($real:tt +i $imaginary:tt) => {
        Complex::new($real, $imaginary)
    };
    ($real:tt -i $imaginary:tt) => {
        Complex::new($real, -($imaginary))
    };
    ($real:tt +i_$imaginary:tt) => {
        Complex::new($real, $imaginary)
    };
    ($real:tt -i_$imaginary:tt) => {
        Complex::new($real, -($imaginary))
    };
}

impl<T> Complex<T>
where
    T: Copy,
{
    pub fn new(r: T, i: T) -> Self {
        Self {
            real: r,
            imaginary: i,
        }
    }

    pub fn real(&self) -> T {
        self.real
    }

    pub fn imaginary(&self) -> T {
        self.imaginary
    }

    pub fn set_real(&mut self, real: T) {
        self.real = real
    }

    pub fn set_imaginary(&mut self, imaginary: T) {
        self.imaginary = imaginary
    }
}

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        (self.real + rhs.real, self.imaginary + rhs.imaginary).into()
    }
}

impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        (self.real + rhs, self.imaginary).into()
    }
}

impl<T> AddAssign<Complex<T>> for Complex<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

impl<T> AddAssign<T> for Complex<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.real += rhs;
    }
}

impl<T> Sub<Complex<T>> for Complex<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Complex<T>) -> Self::Output {
        (self.real - rhs.real, self.imaginary - rhs.imaginary).into()
    }
}

impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        (self.real - rhs, self.imaginary).into()
    }
}

impl<T> SubAssign<Complex<T>> for Complex<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Complex<T>) {
        self.real -= rhs.real;
        self.imaginary -= rhs.imaginary;
    }
}

impl<T> SubAssign<T> for Complex<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.real -= rhs;
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        let real: T = self.real * rhs.real - (self.imaginary * rhs.imaginary);
        let image: T = self.imaginary * rhs.real + (rhs.imaginary * self.real);
        (real, image).into()
    }
}

impl<T: Copy> From<(T, T)> for Complex<T> {
    fn from((r, i): (T, T)) -> Self {
        Self::new(r, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = c!(10 + i 32);
        let v = c!(10 - i 32);
    }
}
