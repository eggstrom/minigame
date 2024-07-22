use super::World;

#[derive(Clone, Copy)]
pub struct Entity(u64);

impl Entity {
    pub fn next(&self) -> Self {
        Entity(self.0 + 1)
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity(0)
    }
}

pub struct EntityHandle<'a> {
    pub(super) world: &'a mut World,
    pub(super) entity: Entity,
}

impl<'a> EntityHandle<'a> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn add_component<T: 'static>(
        &mut self,
        component: T,
    ) -> Result<&'a mut EntityHandle, String> {
        self.world.add_component(component)?;
        Ok(self)
    }
}
