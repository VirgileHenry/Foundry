use crate::utils::collections::packed_array::{PackedArray, IndexedElem};
use std::{cell::UnsafeCell};

/// struct keeping all the components of one type together.
/// This is mostly a wrapper around a PackedArray, so all implementations are redirections.
pub struct ComponentArray<C> {
    /// the component array. We use an UnsafeCell to allow multiple borrows from the component table in iteration.
    comp_arr: UnsafeCell<PackedArray<C>>
}

impl<C> ComponentArray<C> {
    /// Creates a new empty component array.
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new()) };
    }

    /// Creates a new component array with one element in it.
    #[inline]
    pub fn new_with_elem(component: C, index: usize) -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new_with_elem(component, index)) };
    }

    /// Creates a new component array with a vec of elements in it.
    #[inline]
    pub fn new_with_vec(component_vec: Vec<C>, start_index: usize) -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new_with_vec(component_vec, start_index)) };
    }

    /// Add a component in the array.
    #[inline]
    pub fn insert_component(&mut self, component: C, index: usize) -> Option<C> {
        return self.comp_arr.get_mut().insert(component, index);
    }

    /// Append a component at the end of the array.
    #[inline]
    pub fn append_component(&mut self, component: C, index: usize) {
        self.comp_arr.get_mut().append(component, index);
    }

    /// Append a vec of components at the end of the array.
    #[inline]
    pub fn append_components(&mut self, component_vec: Vec<C>, start_index: usize) {
        self.comp_arr.get_mut().append_vec(component_vec, start_index);
    }

    /// Get a reference to a component from the array.
    #[inline]
    pub fn get_component(&self, index: usize) -> Option<&C> {
        unsafe { return (*self.comp_arr.get()).get(index); }
    }

    /// Get a mutable reference to a component from the array.
    #[inline]
    pub fn get_component_mut(&mut self, index: usize) -> Option<&mut C> {
        return self.comp_arr.get_mut().get_mut(index);
    }

    pub unsafe fn unsafe_get_comp_mut(&self, index: usize) -> Option<&mut C> {
        unsafe { return (*self.comp_arr.get()).get_mut(index); }
    }

    /// Remove a component from the array, returning it.
    #[inline]
    pub fn remove_component(&mut self, index: usize) -> Option<C> {
        return self.comp_arr.get_mut().remove(index);
    }

    /// Raw access to the array.
    #[inline]
    pub fn get_array(&self) -> &Vec<IndexedElem<C>> {
        unsafe { return (*self.comp_arr.get()).get_vec(); }
    }

    /// Raw mutable acces to the array.
    #[inline]
    #[allow(dead_code)]
    pub fn get_array_mut(&mut self) -> &mut Vec<IndexedElem<C>> {
        return self.comp_arr.get_mut().get_vec_mut();
    }

    /// Unsafe raw mutable access to the array.
    #[inline]
    pub unsafe fn unsafe_get_array_mut(&self) -> &mut Vec<IndexedElem<C>> {
        return (*self.comp_arr.get()).get_vec_mut();
    }


}