use std::any::TypeId;

use crate::{entity::Entity, world::World};

use super::query_element::QueryElement;

pub trait QueryData<'q> {
    type Item;

    fn inner_type_ids() -> Vec<TypeId>;
    fn get_item(world: &'q World, entity: Entity) -> Self::Item;
}

impl<'q, T> QueryData<'q> for T
where
    T: QueryElement<'q>,
{
    type Item = T::Item;

    fn inner_type_ids() -> Vec<TypeId> {
        vec![T::inner_type_id()].into_iter().flatten().collect()
    }

    fn get_item(world: &'q World, entity: Entity) -> Self::Item {
        <T as QueryElement>::get_item(world, entity)
    }
}

impl<'q, A, B> QueryData<'q> for (A, B)
where
    A: QueryElement<'q>,
    B: QueryElement<'q>,
{
    type Item = (A::Item, B::Item);

    fn inner_type_ids() -> Vec<TypeId> {
        vec![A::inner_type_id(), B::inner_type_id()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn get_item(world: &'q World, entity: Entity) -> Self::Item {
        (A::get_item(world, entity), B::get_item(world, entity))
    }
}
