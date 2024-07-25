#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(trait_alias)]
#![feature(marker_trait_attr)]

mod component;
mod entity;
mod query;
mod resource;
mod system;
mod world;

#[cfg(test)]
mod test_commons;
