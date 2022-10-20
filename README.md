# Foundry

Foundry is a entity-component-system (ecs) library written in rust, with the aim of learning rust and using it for the Gear rust game engine.

It is still in early development, and thus may contain bugs or unimplemented features.

```WARNING : The examples are currently deprecated, and there are a few tweaks to do in order to make them work.```

## How to use it :

### World :

First, you create a ```World``` which represents the whole ecs, and gives the programmer an interface to it:

```rust
let mut world = World::new();
```

### Entity :

Using the world you can now create entities :

```rust
let mut world = World::new();
let entity: Entity = world.create_entity();
let entities: Vec<Entity> = world.create_entities();
```

### Component :

Components can be any struct you want.
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
let mut world = World::new();
let entity: Entity = world.create_entity();
world.add_component(&entity, Position{x:0.0, y:12.4}); // add a component
let pos = world.get_component::<Position>(&entity);
println!("position component : {} {}", pos.x, pos.y);
```

There are also macros that allow creation of one or multiple entities with components already attached to it, that are more efficients :

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

When implementing the ```updatable``` trait, you can use two macros to iterate over any n-uplets of components :

```rust
impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ecs::component_table::ComponentTable, delta: f32) {

    }
}
```

To iterate over any components, use ```iterate_over_components``` and ```iterate_over_components_mut``` macros like so :

```rust
for pos in iterate_over_component!(world.components; Position) {
    /* iterate over all positions */
}

for (pos, vel) in iterate_over_component_mut!(world.components; Position, Velocity) {
    /* iterate with mutability over all positions and velocity */
}
```

With all this, we can for example implement basic gravity :

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

struct PhysicSystem {
    gravity_x: f32,
    gravity_y: f32,
}

impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ecs::component_table::ComponentTable, delta: f32) {
        for (pos, vel) in iterate_over_component_mut!(components; Position, Velocity) {
            vel.vx += self.gravity_x * delta;
            vel.vy += self.gravity_y * delta;
            pos.x += vel.vx * delta;
            pos.y += vel.vy * delta;

            // simple collision
            if pos.y < 0.0 {
                pos.y = -pos.y;
                vel.vy = -0.8 * vel.vy;
            }
        }
    }
}

fn main() {
    
    use std::time::Instant;
    // create ecs and entities
    let mut ecs = World::new();
    let mut entity = create_entities!(ecs; 1_000_000, |i:usize| { return Position{x:0.0, y:5.0}; }, |i:usize| { return Velocity{vx:0.0, vy:0.0}; });

    let physics = PhysicSystem {
        gravity_x: 0.0,
        gravity_y: -9.81,
    };

    let physic_system = System::new(Box::new(physics), UpdateFrequency::Fixed(0.002));

    ecs.register_system(physic_system, 1);

    let mut prev = Instant::now();

    loop {
        let mut delta = prev.elapsed().as_secs_f64();

        ecs.update(delta as f32);
        
        prev = Instant::now();
    }

}

```

Please feel free to play around, and report any bugs or issues, or even optimisations ! 
