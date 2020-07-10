use crate::{Bundle, Component, DynamicBundle, Entity, World};

pub trait WorldBuilderSource {
    fn build(&mut self) -> WorldBuilder;
}

impl WorldBuilderSource for World {
    fn build(&mut self) -> WorldBuilder {
        WorldBuilder {
            world: self,
            current_entity: None,
        }
    }
}

pub struct WorldBuilder<'a> {
    world: &'a mut World,
    pub current_entity: Option<Entity>,
}

impl<'a> WorldBuilder<'a> {
    pub fn entity(&mut self) -> &mut Self {
        self.current_entity = Some(Entity::new());
        self
    }

    pub fn set_entity(&mut self, entity: Entity) -> &mut Self {
        self.current_entity = Some(entity);
        self
    }

    pub fn with<T>(&mut self, component: T) -> &mut Self
    where
        T: Component,
    {
        self.world
            .insert_one(self.current_entity.expect("Cannot add component because the 'current entity' is not set. You should spawn an entity first."), component)
            .unwrap();
        self
    }

    pub fn with_bundle(&mut self, components: impl DynamicBundle) -> &mut Self {
        self.world
            .insert(self.current_entity.expect("Cannot add component because the 'current entity' is not set. You should spawn an entity first."), components)
            .unwrap();
        self
    }

    pub fn spawn_batch<I>(&mut self, components_iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Bundle,
    {
        self.world.spawn_batch(components_iter);
        self
    }

    pub fn spawn(&mut self, components: impl DynamicBundle) -> &mut Self {
        self.current_entity = Some(self.world.spawn(components));
        self
    }

    pub fn spawn_as_entity(&mut self, entity: Entity, components: impl DynamicBundle) -> &mut Self {
        self.world.spawn_as_entity(entity, components);
        self.current_entity = Some(entity);
        self
    }
}
