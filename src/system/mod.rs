use crate::{
    query::{query_data::QueryData, Query},
    world::{world_unsafe_cell::UnsafeWorldCell, World},
};

pub trait System {
    type QueryData<'q>: QueryData<'q>;

    fn run<'q>(&mut self, query: Query<'q, Self::QueryData<'q>>);
}

pub trait RunFn = Fn(*mut (), UnsafeWorldCell);

pub struct StoredSystem {
    state: *mut (),
    run_fn: Box<dyn RunFn>,
}

impl StoredSystem {
    pub fn run(&mut self, world: &World) {
        let cell = unsafe { UnsafeWorldCell::new(world) };
        (self.run_fn)(self.state, cell)
    }

    pub fn from_system<S: System>(system: S) -> Self {
        let state = {
            let boxed = Box::new(system);
            Box::into_raw(boxed) as *mut _
        };

        let run_fn = |this: *mut (), world: UnsafeWorldCell| {
            let (this, world) = unsafe { (&mut *this.cast(), world.get()) };

            let query = world.query();

            S::run(this, query)
        };

        let run_fn = Box::new(run_fn);

        Self { state, run_fn }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        query::fetch::Fetch,
        test_commons::{Health, Mana},
        world::World,
    };

    use super::{StoredSystem, System};

    struct HelloWorldSystem {
        healthy_entities: usize,
    }

    impl System for HelloWorldSystem {
        type QueryData<'q> = Fetch<'q, Health>;

        fn run<'q>(&mut self, query: crate::query::Query<'q, Self::QueryData<'q>>) {
            let count = query.into_iter().filter(|h| h.0 > 50).count();

            self.healthy_entities += count;

            println!(
                "Hello world! Today we've met {count} healthy entities! ({} overall)",
                self.healthy_entities
            );
        }
    }

    #[test]
    fn should_store_and_run_system() {
        let mut world = World::new();

        world.spawn().with_component(Health(100)).build();
        world.spawn().with_component(Health(50)).build();
        world.spawn().with_component(Health(200)).build();
        world.spawn().with_component(Mana(100)).build();

        let mut stored = StoredSystem::from_system(HelloWorldSystem {
            healthy_entities: 0,
        });

        stored.run(&world);
        stored.run(&world);
        stored.run(&world);
        stored.run(&world);
        stored.run(&world);

        unsafe {
            let cast = &*stored.state.cast::<HelloWorldSystem>();
            assert!(cast.healthy_entities == 10)
        }
    }
}
