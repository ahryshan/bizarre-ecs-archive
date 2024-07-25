use std::any::TypeId;

use crate::{
    entity::Entity,
    world::{world_unsafe_cell::UnsafeWorldCell, World},
};

pub trait QueryElement<'q> {
    type Item;

    /// Returns inner type id of the underlying component;
    ///
    /// Must return None if `QueryElement` fetches a `Resource` from `World`
    fn inner_type_id() -> Option<TypeId>;
    fn get_item(world: &'q World, entity: Entity) -> Self::Item;
}
