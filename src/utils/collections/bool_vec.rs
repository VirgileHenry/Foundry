/// More efficient than a Vec<bool>, as it takes 8x less space
/// let's store the bool in a u8 ! each vec component contains 8 of them
pub struct BoolVec {
    pub data: Vec<u8>,
    remaining_space: usize,
}

impl BoolVec {
    pub fn new() -> BoolVec {
        BoolVec {
            data: Vec::new(),
            remaining_space: 0,
        }
    }

    pub fn all_true(size: usize) -> BoolVec {
        // todo : better way to do this
        let mut result = BoolVec::new();
        for i in 0..size {
            result.push(true);
        }
        result
    }

    pub fn push(&mut self, value: bool) {
        if self.remaining_space == 0 {
            self.data.push(0);
            self.remaining_space = 7;
        }
        else {
            self.remaining_space -= 1;
        }
        let last_elem_index = self.data.len() - 1;
        if value {
            let setter_mask = 1 << self.remaining_space;
            self.data[last_elem_index] |= setter_mask;
        }
        else {
            let setter_mask = 255 - (1 << self.remaining_space);
            self.data[last_elem_index] &= setter_mask;
        }
    }

    pub fn append(&mut self, other: BoolVec) {
        for i in 0..other.len() {
            // todo : better way to do this !
            self.push(other.get(i).unwrap());
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() * 8 - self.remaining_space
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        match self.data.get(index / 8) {
            None => None,
            Some(uint8) => {
                let mask = 1 << (7 - index % 8);
                Some(uint8 & mask > 0)
            }
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {

    }
}