# Foundry

Foundry is a entity-component-system (ecs) library written in rust. It has been made to be the foundation of a game engine, and have several unique functionnalities. It is however still under development, and will be optimized and upgraded.

## How to use it :

### World :

First, you create a ```World``` which represents the whole ecs, and gives an interface to build entities, components and systems. A world can be dereferenced to access the inner ```ComponentTable```, which is a key component of this ECS.

```rust
let mut world = World::default();
```

### Entity :

Using the world you can now create entities :

```rust
let mut world = World::default();
let entity: Entity = world.create_entity();
let entities: Vec<Entity> = world.create_entities(100);
```

Entities are an ID, nothing more. The ID is then used to reference the components in the component arrays.

### Component :

Components can be any struct you want. This allow versatility and avoid boilerplate code or overhead to create components. Components can also be composed types, like ```(u32, String)```, but these types are not yet supported by the component iterator macro.
```rust
// a position component
struct Position {
    x: f32,
    y: f32,
}
// a velocity component
struct Velocity {
    vx: f32,
    vy: f32,
}
```

Components can be added, read and removed from entities :

```rust
let mut world = World::default();
let entity: Entity = world.create_entity();
world.add_component(entity, Position{x:0.0, y:12.4}); // add a component
let pos = world.get_component::<Position>(&entity);
println!("position component : {} {}", pos.x, pos.y);
```

There are also macros that allow creation of one or multiple entities with components already attached to it. They are more efficient and should be used in most of the cases.

```rust
let mut world = World::new();
let entity = create_entity!(world.components; Position{x:0.0, y:0.0}, Velocity{vx:0.0, vy:0.0});
// for multiple components, pass in generator functions to give individual components initial values
let entities = create_entities!(world.components; 100, |i:usize|{Position{x:i as f32, y:i as f32}});
```

### Systems

Systems are structure with the ```updatable``` trait that you can register in the world and acts on the components.

```rust
struct PhysicSystem {
    gravity_x: f32,
    gravity_y: f32,
}
```

When implementing the ```updatable``` trait, you can use the ```component_iterator!``` macro to iterate over any tuple of components. This macro is made for simplcity and readability. The general invocation is ```components, mask?; comps+``` 

```rust
impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ecs::component_table::ComponentTable, delta: f32) {
        for pos in component_iterator!(components; Position) {
            /* iterate over all positions */
        }

        for (pos, vel) in component_iterator!(components; mut Position, Velocity) {
            /* iterate with mutability over all positions and velocity */
        }

        for pos in component_iterator!(components 1 << 12; Position) {
            // iterate over all positions on entities that have the 12th layer activated
        }
    }
}
```

Please feel free to play around, and report any bugs, issues or optimisations ! 
