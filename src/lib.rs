#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(trait_alias)]
#![feature(marker_trait_attr)]

mod component;
mod entity;
mod query;
mod resource;
mod world;

#[cfg(test)]
mod test_commons;

#[marker]
pub trait ResourceType {}

pub trait Queryable {
    type ResourceType: ResourceType;
}

pub struct Component;
pub struct Resource;

impl ResourceType for Component {}
impl ResourceType for Resource {}
