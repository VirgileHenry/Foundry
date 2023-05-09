

/// Creates an entity with any number of components.
/// This is way faster than ```create_entity``` then ```add_component```
/// This can be used passing the component table, then the components for exemple : 
/// ```let entity = create_entity!(comp_table; Position{x:0, y:0}, Velocity{vx:0, vy:0});```
#[macro_export]
macro_rules! create_entity {
    ($comp_table:expr) => { 
        foundry::ComponentTable::create_entity($comp_table)
    };
    ($comp_table:expr; $($comp:expr),*) => { {
        // static assertion that components are different
        // otherwise it would create two comps with same entity id
        assert_exclusive_types!($($comp),*);
        let result_entity = foundry::ecs::ComponentTable::create_entity($comp_table);
        $(
            $comp_table.add_comp_to_last(result_entity, $comp);
        )*
        result_entity
    } };
}

/// Creates multiple entities with components.
/// This is way faster than creating entities and adding components to each and everyone of them
/// Uses generators functions for the components allowing to give unique values to the components of the created entities.
/// for example : ```let entities = creates_entities!(comp_table; 1000, |i:usize|{Position{x:i, y:i}});```
#[macro_export]
macro_rules! create_entities {
    ($comp_table:expr; $amount:expr, $($generators:expr),*) => {
        {
            // maybe we should also check the types returned by the generators are different ?
            // could this create safety issues ? -> YES, add_comps_to_last is not safe
            let result_entities = foundry::ComponentTable::create_entities(&mut $comp_table, $amount);
            let start_index = match result_entities.get(0) {Some(entity) => *entity, None => 0};
            $(
                let mut comp_vec = Vec::with_capacity($amount);
                for i in 0..$amount {
                    comp_vec.push($generators(i));
                }
                $comp_table.add_comps_to_last(start_index, comp_vec);
            )*
            result_entities
        }
    };
}