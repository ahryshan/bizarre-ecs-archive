use std::{
    any::TypeId,
    collections::{btree_map, BTreeMap},
};

use error::{ResourceError, ResourceResult};

use crate::component::{
    component_storage::{IntoStoredComponent, StoredComponent},
    Component,
};

pub mod error;

#[derive(Default)]
pub struct Resources {
    map: BTreeMap<TypeId, StoredComponent>,
}

pub trait Resource: Component {}

impl<T: Component> Resource for T {}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<R: IntoStoredComponent>(&mut self, resource: R) -> ResourceResult {
        let resource = resource.into_stored_component();

        if let btree_map::Entry::Vacant(e) = self.map.entry(resource.inner_type_id()) {
            e.insert(resource);
            Ok(())
        } else {
            Err(ResourceError::AlreadyPresent(resource.component_name()))
        }
    }

    pub fn get<R: Resource>(&self) -> ResourceResult<&R> {
        match self.map.get(&R::inner_type_id()) {
            Some(r) => Ok(r.downcast_ref().unwrap()),
            None => Err(ResourceError::NotPresent(R::component_name())),
        }
    }

    pub fn get_mut<R: Resource>(&mut self) -> ResourceResult<&mut R> {
        match self.map.get_mut(&R::inner_type_id()) {
            Some(r) => Ok(r.downcast_mut().unwrap()),
            None => Err(ResourceError::NotPresent(R::component_name())),
        }
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        self.map
            .remove(&R::inner_type_id())
            .map(|r| unsafe { r.into_inner() })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::test_commons::Health;

    use super::{error::ResourceError, Resources};

    #[test]
    fn should_insert_resource() -> Result<()> {
        let mut storage = Resources::new();

        storage.insert(Health(100))?;

        Ok(())
    }

    #[test]
    fn should_err_on_double_insert() -> Result<()> {
        let mut storage = Resources::new();

        storage.insert(Health(100))?;
        match storage.insert(Health(100)) {
            Err(ResourceError::AlreadyPresent(_)) => Ok(()),
            _ => Err(anyhow!(
                "Expected resource storage to prevent double insert"
            )),
        }
    }

    #[test]
    fn should_get_resource() -> Result<()> {
        let mut storage = Resources::new();
        storage.insert(Health(100))?;

        let health = storage.get::<Health>()?;

        assert!(health == &Health(100));

        Ok(())
    }

    #[test]
    fn should_get_resource_mut() -> Result<()> {
        let mut r = Resources::new();
        r.insert(Health(100))?;

        let health = r.get_mut::<Health>()?;

        health.0 = 200;
        let cloned = health.clone();

        let health = r.get::<Health>()?;

        assert!(health == &cloned);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_not_get_nonexistent_resource() {
        let r = Resources::new();
        r.get::<Health>().unwrap();
    }
}
