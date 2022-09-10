use crate::utils::collections::packed_array::{PackedArray, IndexedElem};
use std::{cell::UnsafeCell};


pub struct ComponentArray<C> {
    comp_arr: UnsafeCell<PackedArray<C>>
}

impl<C> ComponentArray<C> {
    #[inline]
    pub fn new() -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new()) };
    }

    #[inline]
    pub fn new_with_elem(component: C, index: usize) -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new_with_elem(component, index)) };
    }

    #[inline]
    pub fn new_with_vec(component_vec: Vec<C>, start_index: usize) -> ComponentArray<C> {
        return ComponentArray { comp_arr: UnsafeCell::new(PackedArray::<C>::new_with_vec(component_vec, start_index)) };
    }

    #[inline]
    pub fn insert_component(&mut self, component: C, index: usize) -> Option<C> {
        return self.comp_arr.get_mut().insert(component, index);
    }

    #[inline]
    pub fn append_component(&mut self, component: C, index: usize) {
        self.comp_arr.get_mut().append(component, index);
    }

    #[inline]
    pub fn append_components(&mut self, component_vec: Vec<C>, start_index: usize) {
        self.comp_arr.get_mut().append_vec(component_vec, start_index);
    }

    #[inline]
    pub fn get_component(&self, index: usize) -> Option<&C> {
        unsafe { return (*self.comp_arr.get()).get(index); }
    }

    #[inline]
    pub fn get_component_mut(&mut self, index: usize) -> Option<&mut C> {
        return self.comp_arr.get_mut().get_mut(index);
    }

    #[inline]
    pub fn remove_component(&mut self, index: usize) -> Option<C> {
        return self.comp_arr.get_mut().remove(index);
    }

    #[inline]
    pub fn get_array(&self) -> &Vec<IndexedElem<C>> {
        unsafe { return (*self.comp_arr.get()).get_vec(); }
    }

    #[inline]
    pub fn get_array_mut(&mut self) -> &mut Vec<IndexedElem<C>> {
        return self.comp_arr.get_mut().get_vec_mut();
    }

    #[inline]
    pub fn unsafe_get_array_mut(&self) -> &mut Vec<IndexedElem<C>> {
        unsafe { return (*self.comp_arr.get()).get_vec_mut(); }
    }


}