use bevy::{
    ecs::{component::Component, system::Command},
    prelude::*,
};
use std::marker::PhantomData;

/// Extension methods for [`Commands`](bevy::ecs::system::Commands)
pub trait CommandsExt {
    fn init_resource<T: Component + FromWorld>(&mut self);
}

impl CommandsExt for Commands<'_> {
    fn init_resource<T: Component + FromWorld>(&mut self) {
        self.add(InitResource {
            _phantom_data: PhantomData::<T>,
        })
    }
}

pub struct InitResource<T: Component + FromWorld> {
    _phantom_data: PhantomData<T>,
}

impl<T: Component + FromWorld> Command for InitResource<T> {
    fn write(self: Box<Self>, world: &mut World) {
        if !world.contains_resource::<T>() {
            let resource = T::from_world(world);
            world.insert_resource(resource);
        }
    }
}
