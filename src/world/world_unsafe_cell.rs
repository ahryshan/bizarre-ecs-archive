use std::ptr;

use super::World;

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell(*mut World);

impl UnsafeWorldCell {
    pub unsafe fn new(world: &World) -> Self {
        Self(ptr::from_ref(world).cast_mut())
    }

    pub unsafe fn get(&self) -> &World {
        self.0.as_ref().unwrap()
    }

    pub unsafe fn get_mut(&mut self) -> &mut World {
        self.0.as_mut().unwrap()
    }
}
