use std::{
    ops,
    default::Default,
};

pub type VecF = Vector<f32>;

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T>
    where T: Clone + Default {

    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, Default::default());

        Vector {
            data
        }
    }

    pub fn from_slice(values: &[T]) -> Self {
        let data = Vec::from(values);

        Vector {
            data
        }
    }
}

impl<T> Vector<T>
    where T: Copy + Default + ops::Add<Output=T> + ops::Mul<Output=T> {

    pub fn dot(&self, rhs: &Vector<T>) -> T {
        self.data.iter().zip(rhs.data.iter())
            .map(|(&lv, &rv)| lv * rv)
            .fold(Default::default(), |acc, v| acc + v)
    }
}

impl Vector<f32> {
    pub fn norm(&self) -> f32 {
        self.data.iter()
            .map(|v| v * v)
            .fold(0.0_f32, |acc, v| acc + v)
            .sqrt()
    }

    pub fn normalize(&self) -> Vector<f32> {
        let norm = self.norm();
        let data = self.data.iter()
            .map(|v| v / norm)
            .collect();

        Vector {
            data
        }
    }
}

impl<T> Clone for Vector<T>
    where T: Clone {

    fn clone(&self) -> Self {
        let data = self.data.clone();

        Vector {
            data
        }
    }
}

impl<T> ops::Index<i32> for Vector<T> {
    type Output = T;

    fn index(&self, axis: i32) -> &Self::Output {
        &self.data[axis as usize]
    }
}

impl<T> ops::IndexMut<i32> for Vector<T> {
    fn index_mut(&mut self, axis: i32) -> &mut Self::Output {
        self.data.index_mut(axis as usize)
    }
}

impl<T> ops::Sub for &Vector<T>
    where T: Copy + ops::Sub<Output=T> {

    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let data = self.data.iter().zip(rhs.data.iter())
            .map(|(&lv, &rv)| lv - rv)
            .collect();
        Vector {
            data
        }
    }
}

impl<T> ops::Add for &Vector<T>
    where T: Copy + ops::Add<Output=T> {

    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let data = self.data.iter().zip(rhs.data.iter())
            .map(|(&lv, &rv)| lv + rv)
            .collect();
        Vector {
            data
        }
    }
}

impl<T> ops::Mul<T> for &Vector<T>
    where T: Copy + ops::Mul<Output=T> {

    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let data = self.data.iter()
            .map(|&v| v * rhs)
            .collect();
        Vector {
            data
        }
    }
}

pub fn reflect(i: &VecF, n: &VecF) -> VecF {
    i - &(&(n * 2.0) * (i.dot(n)))
}

