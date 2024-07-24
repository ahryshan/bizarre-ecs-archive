use crate::{
    component::{error::ComponentResult, Component, Components},
    entity::{builder::EntityBuilder, entities::Entities, error::EntityResult, Entity},
    resource::{error::ResourceResult, Resource, Resources},
};

#[derive(Default)]
pub struct World {
    entities: Entities,
    components: Components,
    resources: Resources,
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

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Option<C> {
        self.components.remove(entity)
    }

    pub fn create_entity(&mut self) -> Entity {
        let (entity, reused) = self.entities.spawn();

        if !reused {
            self.components.expand();
        }

        entity
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> ResourceResult {
        self.resources.insert(resource)
    }

    pub fn get_resource<R: Resource>(&self) -> ResourceResult<&R> {
        self.resources.get()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> ResourceResult<&mut R> {
        self.resources.get_mut()
    }

    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove()
    }
}
