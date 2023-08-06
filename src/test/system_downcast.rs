use crate::AsAny;

use crate::{Updatable, System};



#[test]
/// Test the ability to insert a system and downcast it back.
fn test_system_downcast() {
    #[derive(AsAny)]
    struct MySystem;

    impl Updatable for MySystem {
        fn update(&mut self, _components: &mut crate::ComponentTable, _delta: f32) { }
    }

    let syst = System::new(MySystem, crate::UpdateFrequency::PerFrame);

    assert!(syst.try_get_updatable::<MySystem>().is_some());

}