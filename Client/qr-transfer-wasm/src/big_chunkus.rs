#[derive(Clone)]
struct Chunk {
    data: Vec<u8>,
    total: usize,
    part: usize,
    is_placeholder: bool,
}

trait ChunkValidator {
    fn get_chunk(&self, index: usize) -> Option<&Chunk>;
}

impl ChunkValidator for Vec<Chunk> {
    fn get_chunk(&self, index: usize) -> Option<&Chunk> {
        let chunk = self.get(index);
        if chunk.is_none() {
            return None;
        }
        let chunk = chunk.unwrap();
        if chunk.is_placeholder {
            return None;
        } else {
            Some(chunk)
        }
    }
}

struct BigChunkus {
    chunks: Vec<Chunk>,
}

impl BigChunkus {
    pub fn new() -> BigChunkus {
        BigChunkus { chunks: Vec::new() }
    }

    pub fn mount() -> Result<Vec<u8>, &'static str> {
        todo!()
    }

    fn valid_chunk(&self, chunk: &Chunk) -> bool {
        let last = self.chunks.last();
        if last.is_none() {
            return true;
        }
        let last = last.unwrap();

        if last.total != chunk.total {
            return false;
        }
        let target = self.chunks.get_chunk(chunk.part);
        if target.is_none() {
            return true;
        }

        return false;
    }

    pub fn add(&mut self, chunk: Chunk) -> Result<usize, &'static str> {
        let index = chunk.part.clone();
        if self.valid_chunk(&chunk) {
            // Resize the chunks vector if necessary
            if self.chunks.len() <= index {
                self.chunks.resize(
                    index + 1,
                    Chunk {
                        data: Vec::new(),
                        total: 0,
                        part: 0,
                        is_placeholder: true,
                    },
                );
            }
            self.chunks[index] = chunk;
            Ok(index)
        } else {
            Err("Invalid chunk")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_first_chunk() {
        let mut big_chunkus = BigChunkus::new();
        let chunk = Chunk {
            data: vec![1, 2, 3],
            total: 3,
            part: 0,
        };

        let result = big_chunkus.add(chunk.clone());
        assert_eq!(result, Ok(0));
        assert_eq!(big_chunkus.chunks[0].data, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_multiple_chunks() {
        let mut big_chunkus = BigChunkus::new();

        let chunk1 = Chunk {
            data: vec![1, 2, 3],
            total: 3,
            part: 0,
        };
        let chunk2 = Chunk {
            data: vec![4, 5, 6],
            total: 3,
            part: 1,
        };

        big_chunkus.add(chunk1.clone()).unwrap();
        big_chunkus.add(chunk2.clone()).unwrap();

        assert_eq!(big_chunkus.chunks[0].data, vec![1, 2, 3]);
        assert_eq!(big_chunkus.chunks[1].data, vec![4, 5, 6]);
    }

    #[test]
    fn test_invalid_chunk() {
        let mut big_chunkus = BigChunkus::new();

        let chunk1 = Chunk {
            data: vec![1, 2, 3],
            total: 3,
            part: 0,
        };
        let chunk2 = Chunk {
            data: vec![4, 5, 6],
            total: 4, // Different total
            part: 1,
        };

        big_chunkus.add(chunk1).unwrap();
        let result = big_chunkus.add(chunk2);

        assert_eq!(result, Err("Invalid chunk"));
    }

    #[test]
    fn test_add_chunk_same_part() {
        let mut big_chunkus = BigChunkus::new();

        let chunk1 = Chunk {
            data: vec![1, 2, 3],
            total: 3,
            part: 0,
        };
        let chunk2 = Chunk {
            data: vec![4, 5, 6],
            total: 3,
            part: 0, // Same part as chunk1
        };

        big_chunkus.add(chunk1.clone()).unwrap();
        let result = big_chunkus.add(chunk2.clone());

        assert_eq!(result, Err("Invalid chunk"));
        assert_eq!(big_chunkus.chunks[0].data, vec![1, 2, 3]);
    }

    #[test]
    fn test_iter_data() {
        let mut big_chunkus = BigChunkus::new();

        let chunk1 = Chunk {
            data: vec![1, 2, 3],
            total: 3,
            part: 0,
        };
        let chunk2 = Chunk {
            data: vec![7, 8, 9],
            total: 3,
            part: 2, // Same part as chunk1
        };

        let result = big_chunkus.add(chunk1);
        assert!(result.is_ok());
        let result = big_chunkus.add(chunk2);
        assert!(result.is_ok());

        let item = big_chunkus.chunks.get(0);
        assert!(item.is_some());
        let item = big_chunkus.chunks.get(1);
        assert!(item.is_some());

        let item = big_chunkus.chunks.get(2);
        assert!(item.is_some())
    }
}
