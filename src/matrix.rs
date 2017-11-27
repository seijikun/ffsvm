use std;

/// Basic Matrix we use for fast SIMD and parallel operations
#[derive(Debug)]
pub struct Matrix<T> where
    T : std::fmt::Debug,
    T : std::marker::Copy,
    T : std::clone::Clone
{
    pub vectors: usize,
    pub attributes: usize,
    pub data: Vec<T>
}


impl<T> Matrix<T> where
    T : std::fmt::Debug,
    T : std::marker::Copy,
    T : std::clone::Clone
{
    /// Creates a new Matrix
    pub fn new(vectors: usize, attributes: usize, default: T) -> Matrix<T> {
        Matrix::<T> {
            vectors,
            attributes,
            data: vec![default; (vectors * attributes)],
        }
    }

    #[inline]
    pub fn get_vector(&self, index_vector: usize) -> &[T] {
        let start_index = self.offset(index_vector, 0);
        &self.data[start_index..start_index + self.attributes]
    }

    #[inline]
    pub fn offset(&self, index_vector: usize, index_attribute: usize) -> usize {
        ((index_vector * self.attributes) + index_attribute)
    }
    
    #[inline]
    pub fn set(&mut self, index_vector: usize, index_attribute: usize, value: T) {
        let  index = self.offset(index_vector, index_attribute);
        self.data[index] = value;
    }

    #[inline]
    pub fn get(&self, index_vector: usize, index_attribute: usize) -> T {
        let  index = self.offset(index_vector, index_attribute);
        self.data[index]
    }
}