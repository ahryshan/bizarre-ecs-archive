use crate::{
    component::{error::ComponentResult, Component, Components},
    entity::{builder::EntityBuilder, entities::Entities, error::EntityResult, Entity},
};

#[derive(Default)]
pub struct World {
    entities: Entities,
    components: Components,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub fn kill(&mut self, entity: Entity) -> EntityResult {
        self.entities.kill(entity)?;
        self.components.remove_entity(entity);
        Ok(())
    }

    pub fn insert_component<C: Component>(
        &mut self,
        entity: Entity,
        component: C,
    ) -> ComponentResult {
        self.components.insert(entity, component)
    }

    pub fn register_component<C: Component>(&mut self) {
        self.components.register::<C>()
    }

    pub fn create_entity(&mut self) -> Entity {
        let (entity, reused) = self.entities.spawn();

        if !reused {
            self.components.expand();
        }

        entity
    }
}
