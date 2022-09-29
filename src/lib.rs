use std::{
    fmt::{Debug, Write},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Complex<T: Copy> {
    real: T,
    imaginary: T,
}

impl<T> Debug for Complex<T>
where
    T: Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        self.real.fmt(f)?;
        f.write_str(", ")?;
        self.imaginary.fmt(f)?;
        f.write_char('i')?;
        f.write_char(')')?;
        Ok(())
    }
}

#[macro_export]
macro_rules! c {
    ($real:tt +i $imaginary:tt) => {
        Complex::new($real, $imaginary)
    };
    ($real:tt -i $imaginary:tt) => {
        Complex::new($real, -($imaginary))
    };
    (-$real:tt +i $imaginary:tt) => {
        Complex::new(-$real, $imaginary)
    };
    (-$real:tt -i $imaginary:tt) => {
        Complex::new(-$real, -($imaginary))
    };
}

impl<T> Complex<T>
where
    T: Copy,
{
    pub const fn new_const(r: T, i: T) -> Self {
        Self {
            real: r,
            imaginary: i,
        }
    }

    #[inline]
    pub fn new(r: T, i: T) -> Self {
        Self {
            real: r,
            imaginary: i,
        }
    }

    #[inline]
    pub fn real(&self) -> T {
        self.real
    }

    #[inline]
    pub fn imaginary(&self) -> T {
        self.imaginary
    }

    #[inline]
    pub fn set_real(&mut self, real: T) {
        self.real = real
    }

    #[inline]
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
        let imaginary: T = self.imaginary * rhs.real + (rhs.imaginary * self.real);
        (real, imaginary).into()
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    type Output = Complex<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let real: T = self.real * rhs;
        let imaginary: T = self.imaginary * rhs;
        (real, imaginary).into()
    }
}

impl<T> MulAssign<Complex<T>> for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: Complex<T>) {
        let real: T = self.real * rhs.real - (self.imaginary * rhs.imaginary);
        let imaginary: T = self.imaginary * rhs.real + (rhs.imaginary * self.real);
        self.real = real;
        self.imaginary = imaginary;
    }
}

impl<T> MulAssign<T> for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        let real: T = self.real * rhs;
        let imaginary: T = self.imaginary * rhs;
        self.real = real;
        self.imaginary = imaginary;
    }
}

impl<T> Div<Complex<T>> for Complex<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sub<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Complex<T>) -> Self::Output {
        // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2) +((bc-ad)/(c^2+d^2))i
        let a = self.real;
        let b = self.imaginary;
        let c = rhs.real;
        let d = rhs.imaginary;

        let real = ((a * c) + (b * d)) / ((c * c) + (d * d));
        let imaginary = ((b * c) - (a * d)) / ((c * c) + (d * d));
        (real, imaginary).into()
    }
}

impl<T> Div<T> for Complex<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sub<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2) +((bc-ad)/(c^2+d^2))i
        let a = self.real;
        let b = self.imaginary;
        let c = rhs;

        let real = a / c;
        let imaginary = b / c;

        (real, imaginary).into()
    }
}

impl<T> DivAssign<Complex<T>> for Complex<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sub<Output = T>,
{
    fn div_assign(&mut self, rhs: Complex<T>) {
        // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2) +((bc-ad)/(c^2+d^2))i
        let a = self.real;
        let b = self.imaginary;
        let c = rhs.real;
        let d = rhs.imaginary;

        let real = ((a * c) + (b * d)) / ((c * c) + (d * d));
        let imaginary = ((b * c) - (a * d)) / ((c * c) + (d * d));

        self.real = real;
        self.imaginary = imaginary;
    }
}

impl<T> DivAssign<T> for Complex<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sub<Output = T>,
{
    fn div_assign(&mut self, rhs: T) {
        // (a+bi)/(c+di)=(ac+bd)/(c^2+d^2) +((bc-ad)/(c^2+d^2))i
        let a = self.real;
        let b = self.imaginary;
        let c = rhs;

        let real = a / c;
        let imaginary = b / c;

        self.real = real;
        self.imaginary = imaginary;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        (-self.real, -self.imaginary).into()
    }
}

impl<T: Copy> From<(T, T)> for Complex<T> {
    fn from((r, i): (T, T)) -> Self {
        Self::new(r, i)
    }
}

impl<T> From<T> for Complex<T>
where
    T: Copy + Default,
{
    fn from(r: T) -> Self {
        Self::new(r, T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // https://mathsolver.microsoft.com/zh/solve-problem/

        let a = c!(3 + i 2);
        let b = c!(2 - i 3);

        let c = a * b;
        assert_eq!(c, (12, -9 + 4).into());

        let c = a + b;
        assert_eq!(c, c!(5 - i 1));

        let c = a - b;
        assert_eq!(c, c!(1 + i 5));

        assert_eq!(b - a, -c);

        assert_eq!(a / b, (0, 1).into());
    }
}
