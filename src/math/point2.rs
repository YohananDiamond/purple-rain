use num_traits::Signed;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Copy and Clone are manually implemented for this struct.
#[derive(Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Copy for Point2<T> {}

impl<T: Clone> Clone for Point2<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_point<E>(other: Point2<E>) -> Self
    where
        E: Into<T>,
    {
        Self {
            x: other.x.into(),
            y: other.y.into(),
        }
    }

    pub fn into_point<E>(self) -> Point2<E>
    where
        T: Into<E>,
    {
        Point2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T> PartialEq for Point2<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Point2<T>
where
    T: Signed,
{
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn x_abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y,
        }
    }

    pub fn y_abs(self) -> Self {
        Self {
            x: self.x,
            y: self.y.abs(),
        }
    }
}

impl<T> Add for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point2<T>
where
    T: Add<Output = T> + Clone,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() + other.x,
            y: self.y.clone() + other.y,
        };
    }
}

impl<T> Sub for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Point2<T>
where
    T: Sub<Output = T> + Clone,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() - other.x,
            y: self.y.clone() - other.y,
        };
    }
}

impl<T> Div for Point2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign for Point2<T>
where
    T: Div<Output = T> + Clone,
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() / other.x,
            y: self.y.clone() / other.y,
        };
    }
}

impl<T> Mul for Point2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> MulAssign for Point2<T>
where
    T: Mul<Output = T> + Clone,
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x.clone() * other.x,
            y: self.y.clone() * other.y,
        };
    }
}

impl<T> Into<(T, T)> for Point2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> From<(T, T)> for Point2<T> {
    fn from(other: (T, T)) -> Self {
        let (x, y) = other;
        Self { x, y }
    }
}

impl<T> Into<[T; 2]> for Point2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T> From<&[T; 2]> for Point2<T>
where
    T: Copy,
{
    fn from(other: &[T; 2]) -> Self {
        let x = other[0];
        let y = other[1];
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2_add() {
        assert_eq!(
            Point2::<i32>::new(1, 2) + Point2::<i32>::new(2, 1),
            Point2::<i32>::new(3, 3)
        );
    }

    #[test]
    fn p2_mul() {
        assert_eq!(
            Point2::<i32>::new(1, 2) * Point2::<i32>::new(2, 1),
            Point2::<i32>::new(2, 2)
        );
    }

    #[test]
    fn p2_div() {
        assert_eq!(
            Point2::<i32>::new(10, 20) / Point2::<i32>::new(5, 5),
            Point2::<i32>::new(2, 4)
        );
    }

    #[test]
    fn p2_sub() {
        assert_eq!(
            Point2::<i32>::new(2, 2) - Point2::<i32>::new(2, 3),
            Point2::<i32>::new(0, -1)
        );
    }

    #[test]
    fn p2_eq() {
        assert_eq!(Point2::<i32>::new(1, 2), Point2::<i32>::new(1, 2));
    }
}
