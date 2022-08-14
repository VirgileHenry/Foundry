// data structure with the goals of :
// - fast iteration
// - fast insertion, look up and removal
// - ordered

pub struct PackedArray<T> {
    data: Vec<IndexedElem<T>>,
}

pub struct IndexedElem<T> {
    pub index: usize,
    pub elem: T
}

impl<T> IndexedElem<T> {
    pub fn new(elem: T, index: usize) -> IndexedElem<T> {
        return IndexedElem {
            index: index,
            elem: elem
        };
    }
}

// most impl are wrapper around exisiting Vec impl
impl<T> PackedArray<T> {
    // create a new, empty packed array of type T.
    pub fn new() -> PackedArray<T> {
        return PackedArray { data: Vec::<IndexedElem<T>>::new() };
    }

    pub fn new_with_elem(elem: T, index: usize) -> PackedArray<T> {
        return PackedArray { data: vec![IndexedElem::new(elem, index)] };
    }

    // insert an element to the packed array. If an element with similar index exists, it is replaced and returned
    pub fn insert(&mut self, elem: T, index: usize) -> Option<T> {
        // binary search
        let new_elem = IndexedElem::new(elem, index);
        if self.data.len() == 0 {
            // empty vec
            self.data.push(new_elem);
            return None;
        }
        let mut min_bound: usize = 0;
        let mut max_bound: usize = self.data.len();
        while max_bound - min_bound > 1 {
            let center = (min_bound + max_bound) / 2;
            if self.data[center].index > index {
                max_bound = center;
            }
            else {
                min_bound = center;
            }
        }
        // if an element with the same index exists, it is at min_bound
        if self.data[min_bound].index >= index {
            // use replace to put the new element where the old one was, return the old one
            return Some(std::mem::replace(&mut self.data[min_bound], new_elem).elem);
        }
        else {
            // otherwise, insert at max_bound (which is min_bound +1)
            self.data.insert(max_bound, new_elem);
            return None;
        }
    }

    pub fn append(&mut self, elem: T, index: usize) {
        // consider the index is the biggest elem
        if index <= self.data[self.data.len()-1].index {
            panic!("Unable to append to packed array : index was not the last elem. Consider using insert.");
        }
        self.data.push(IndexedElem::new(elem, index));
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        // binary search
        let mut min_bound: usize = 0;
        let mut max_bound: usize = self.data.len();
        while max_bound - min_bound > 1 {
            let center = (min_bound + max_bound) / 2;
            if self.data[center].index > index {
                max_bound = center;
            }
            else {
                min_bound = center;
            }
        }
        // if an element with the index exists, it is at min_bound
        if self.data[min_bound].index == index {
            return Some(self.data.remove(min_bound).elem);
        }
        return None;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // binary search
        let mut min_bound: usize = 0;
        let mut max_bound: usize = self.data.len();
        while max_bound - min_bound > 1 {
            let center = (min_bound + max_bound) / 2;
            if self.data[center].index > index {
                max_bound = center;
            }
            else {
                min_bound = center;
            }
        }
        // if an element with the index exists, it is at min_bound
        if self.data[min_bound].index == index {
            return Some(&self.data[min_bound].elem);
        }
        else {
            return None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        // binary search
        let mut min_bound: usize = 0;
        let mut max_bound: usize = self.data.len();
        while max_bound - min_bound > 1 {
            let center = (min_bound + max_bound) / 2;
            if self.data[center].index > index {
                max_bound = center;
            }
            else {
                min_bound = center;
            }
        }
        // if an element with the index exists, it is at min_bound
        if self.data[min_bound].index == index {
            return Some(&mut self.data[min_bound].elem);
        }
        else {
            return None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<IndexedElem<T>> {
        return self.data.iter();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<IndexedElem<T>> {
        return self.data.iter_mut();
    }
}

