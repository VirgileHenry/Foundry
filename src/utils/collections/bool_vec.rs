/// More efficient than a Vec<bool>, as it takes 8x less space
/// let's store the bool in a u8 ! each vec component contains 8 of them.
/// At it's core, this is a wrapper around Vec<u8> where each bits represent a bool.
pub struct BoolVec {
    pub data: Vec<u8>,
    remaining_space: usize,
}

impl BoolVec {
    /// creates a new bool vec
    pub fn new() -> BoolVec {
        BoolVec {
            data: Vec::new(),
            remaining_space: 0,
        }
    }

    /// Creates a new BoolVec where all bits are set to true
    pub fn all_true(size: usize) -> BoolVec {
        // todo : better way to do this
        let mut result = BoolVec::new();
        for i in 0..size {
            result.push(true);
        }
        result
    }

    /// Add a value at the end of the bool vec
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

    /// append another bool vec (under construction, can be done a lot better)
    pub fn append(&mut self, other: BoolVec) {
        for i in 0..other.len() {
            // todo : better way to do this !
            self.push(other.get(i).unwrap());
        }
    }

    /// Give the number of elements in the bool vec
    pub fn len(&self) -> usize {
        self.data.len() * 8 - self.remaining_space
    }

    /// Get the value at the given index
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
        match self.data.get_mut(index / 8) {
            Some(uint8) => {
                if value { // set the bit to 1
                    let mask: u8 = 1 << (7 - index % 8);
                    *uint8 = *uint8 | mask;
                }
                else {
                    let mask: u8 = !(1 << (7 - index % 8));
                    *uint8 = *uint8 & mask;
                }
            }
            _ => {},
        }
    }


}