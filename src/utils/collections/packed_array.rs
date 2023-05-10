// data structure with the goals of :
// - fast iteration
// - fast insertion, look up and removal
// - ordered

/// A packed array is a replacement for a huge array where lots of slots would be empty.
/// It's actually a vec of indexed elements, where the index would represent where the elment would be in the real array.
pub struct PackedArray<T> {
    /// The vec of indexed elements
    data: Vec<IndexedElem<T>>,
}

/// Wrapper for any type with an index.
pub struct IndexedElem<T> {
    /// The index
    index: usize,
    /// the element
    elem: T
}

impl<T> IndexedElem<T> {
    /// Create a indexed element from an element and an index.
    pub fn new(elem: T, index: usize) -> IndexedElem<T> {
        return IndexedElem {
            index,
            elem
        };
    }

    #[inline]
    pub fn elem(&self) -> &T {
        &self.elem
    }

    #[inline]
    pub fn elem_mut(&mut self) -> &mut T {
        &mut self.elem
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}

// most impl are wrapper around exisiting Vec impl
impl<T> PackedArray<T> {
    /// create a new, empty packed array of type T.
    pub fn new() -> PackedArray<T> {
        return PackedArray { data: Vec::<IndexedElem<T>>::new() };
    }

    /// Creates a new packed array with one element in it.
    pub fn new_with_elem(elem: T, index: usize) -> PackedArray<T> {
        return PackedArray { data: vec![IndexedElem::new(elem, index)] };
    }

    /// Creates a new packed array with multiple elements in it.
    pub fn new_with_vec(vec: Vec<T>, start_index: usize) -> PackedArray<T> {
        let mut start_vec = Vec::<IndexedElem<T>>::with_capacity(vec.len());
        let mut index: usize = 0;
        for elem in vec.into_iter() {
            start_vec.push(IndexedElem{elem: elem, index:index + start_index});
            index += 1;
        }
        return PackedArray { data: start_vec };
    }

    /// insert an element to the packed array. If an element with similar index exists, it is replaced and returned
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

    /// Append an element at the end of the array. To keep the indexed orderer, we must be sure this elem is the last one.
    /// This is faster than inserting in the array however.
    pub fn append(&mut self, elem: T, index: usize) {
        // consider the index is the biggest elem
        self.data.push(IndexedElem::new(elem, index));
    }

    /// Append a vec of element at the end of the array. To keep the indexed orderer, we must be sure this elem is the last one.
    /// This is faster than inserting in the array however.
    pub fn append_vec(&mut self, elems: Vec<T>, start_index: usize) {
        let mut to_add_vec = Vec::<IndexedElem<T>>::with_capacity(elems.len());
        let mut index = 0;
        for elem in elems.into_iter() {
            to_add_vec.push(IndexedElem{elem:elem, index:start_index+index});
            index += 1;
        }
        self.data.append(&mut to_add_vec);
    }

    /// remove an elements from the array and return it.
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

    /// Get a reference to an element of the array.
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

    /// Get a mutable reference to an element of the array
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

    /// Raw access to the vec.
    pub fn get_vec(&self) -> &Vec<IndexedElem<T>> {
        return &self.data;
    }

    /// Raw mutable access to the vec
    pub fn get_vec_mut(&mut self) -> &mut Vec<IndexedElem<T>> {
        return &mut self.data;
    }
}

