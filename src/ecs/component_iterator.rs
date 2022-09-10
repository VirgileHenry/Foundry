
#[macro_export]
macro_rules! fn_internal_get_next_elem {
    ($elem_type:ty; $first:path, $($elems:path),*) => {
        fn macro_generated_return_next(elem: $elem_type) -> $elem_type {
            match elem {
                $first => 
                $($elems, $elems =>)*
                $first
            }
        }

        fn macro_generated_reset() -> $elem_type {
            $first
        }
    }
}



#[macro_export]
macro_rules! iterate_over_component {
    ($components:expr; $($comp:ident),+) => {
        {
            use crate::ecs::component_table::ComponentTable;
            use crate::utils::collections::packed_array::IndexedElem;
            use std::slice::Iter;


            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                $(
                    $comp: std::iter::Peekable<Iter<'a, IndexedElem<$comp>>>
                ),+
            }

            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
            
            impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    while match &self.$comp.peek() {
                                        None => return None,
                                        Some(elem) => {
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index;
                                                false
                                            }
                                            else if elem.index == self.current_entity {
                                                self.current_component = macro_generated_return_next(self.current_component);
                                                false
                                            }
                                            else {
                                                true
                                            }
                                        }
                                    } {
                                        self.$comp.next();
                                    }
                                }
                            )+
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = ($(
                                    match self.$comp.next() {
                                        Some(elem) => & elem.elem,
                                        None => return None,
                                    }
                                ),+);
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }

            let mut result = MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: macro_generated_reset(),
                $(
                    $comp: match ComponentTable::get_component_array_mut::<$comp>(&$components) {
                        Some(comp_arr) => comp_arr.iter().peekable(),
                        None => [].iter().peekable(),
                    }
                ),+
            };

            result

        }
    };
}



#[macro_export]
macro_rules! iterate_over_component_mut {
    ($components:expr; $($comp:ident),+) => {
        {
            use crate::ecs::component_table::ComponentTable;
            use crate::utils::collections::packed_array::IndexedElem;
            use std::slice::IterMut;


            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                $(
                    $comp: std::iter::Peekable<IterMut<'a, IndexedElem<$comp>>>
                ),+
            }

            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
            
            impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a mut $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    while match &self.$comp.peek() {
                                        None => return None,
                                        Some(elem) => {
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index;
                                                false
                                            }
                                            else if elem.index == self.current_entity {
                                                self.current_component = macro_generated_return_next(self.current_component);
                                                false
                                            }
                                            else {
                                                true
                                            }
                                        }
                                    } {
                                        self.$comp.next();
                                    }
                                }
                            )+
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = ($(
                                    match self.$comp.next() {
                                        Some(elem) => & mut elem.elem,
                                        None => return None,
                                    }
                                ),+);
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }

            let mut result = MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: macro_generated_reset(),
                $(
                    $comp: match ComponentTable::get_component_array_mut::<$comp>(&$components) {
                        Some(comp_arr) => comp_arr.iter_mut().peekable(),
                        None => [].iter_mut().peekable(),
                    }
                ),+
            };

            result

        }
    };
}

/*
some code to make the iterator with functions :

fn main() {
    let mut v: Vec<(f32, usize)> = vec![(2.4, 0), (3.5, 1), (4.2, 4)];
    let mut w: Vec<(u64, usize)> = vec![(2, 1), (3, 4), (4, 5)];
    eprintln!("v: {:?} w: {:?}", v, w);

    let mut iv = v.iter_mut().peekable();
    let mut ip = w.iter_mut().peekable();

    let merged = std::iter::from_fn(|| loop {
        let advance_on = if let Some(i1) = iv.peek().map(|(_, i)| i) {
            if let Some(i2) = ip.peek().map(|(_, i)| i) {
                if i1 < i2 {
                    1
                } else if i1 > i2 {
                    2
                } else {
                    let a = iv.next().unwrap();
                    let b = ip.next().unwrap();
                    return Some((&mut a.0, &mut b.0));
                }
            } else {
                return None;
            }
        } else {
            return None;
        };
        if advance_on == 1 {
            iv.next();
        } else {
            ip.next();
        }
    });

    for (pos, speed) in merged {
        *pos += 1.0;
        *speed += 2;
    }
    eprintln!("v: {:?} w: {:?}", v, w);
}

*/