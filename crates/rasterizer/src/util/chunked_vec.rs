use std::ops::{Index, IndexMut};

pub struct ChunkedVec<T, const CHUNK_SIZE: usize> {
    chunks: Vec<Vec<T>>,
}

impl<T, const CHUNK_SIZE: usize> ChunkedVec<T, CHUNK_SIZE> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chunks: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.chunks.len() == 0 || self.chunks.last().unwrap().len() == CHUNK_SIZE {
            let mut chunk = Vec::with_capacity(CHUNK_SIZE);
            chunk.push(elem);
            self.chunks.push(chunk);
        } else {
            self.chunks.last_mut().unwrap().push(elem);
        }
    }

    pub fn len(&self) -> usize {
        if self.chunks.len() == 0 {
            0
        } else {
            (self.chunks.len() - 1) * CHUNK_SIZE + self.chunks.last().unwrap().len()
        }
    }

    pub fn chunks(&self) -> &Vec<Vec<T>> {
        &self.chunks
    }
}

impl<T, const CHUNK_SIZE: usize> Index<usize> for ChunkedVec<T, CHUNK_SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chunks[index / CHUNK_SIZE][index % CHUNK_SIZE]
    }
}

impl<T, const CHUNK_SIZE: usize> IndexMut<usize> for ChunkedVec<T, CHUNK_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chunks[index / CHUNK_SIZE][index % CHUNK_SIZE]
    }
}
