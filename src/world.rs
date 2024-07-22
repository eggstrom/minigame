use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    mem, thread,
    time::{Duration, Instant},
};

use crate::SharedState;

pub struct World {
    ticks_per_second: u16,
    tick_instant: Instant,
    next_entity: Entity,
    entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<u8>>,
    _component_lookup: HashMap<TypeId, Vec<Entity>>,
    systems: HashMap<TypeId, Vec<fn()>>,
}

impl World {
    pub fn new(ticks_per_second: u16) -> Self {
        World {
            ticks_per_second,
            tick_instant: Instant::now(),
            next_entity: Entity::default(),
            entities: vec![],
            components: HashMap::new(),
            _component_lookup: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn update(&mut self, state: &SharedState) -> Result<(), String> {
        // let instant = Instant::now();

        // let mut draw_data = vec![];
        // let mut requests = vec![];
        //
        // for entity in self.entities.iter_mut() {
        //     let (dd, req) = entity.update();
        //     dd.map(|mut dd| draw_data.append(&mut dd));
        //     req.map(|mut req| requests.append(&mut req));
        // }
        //
        // state.set_draw_data(draw_data)?;
        // state.push_requests(&mut requests)?;

        // log::info!("World update took {}us", instant.elapsed().as_micros());

        self.await_next_tick();
        Ok(())
    }

    fn await_next_tick(&mut self) {
        let tick_duration = Duration::from_secs(1) / self.ticks_per_second as u32;
        thread::sleep(tick_duration.saturating_sub(self.tick_instant.elapsed()));
        self.tick_instant = Instant::now();
    }

    pub fn register_component<T: 'static>(&mut self) -> Result<(), String> {
        self.components
            .insert(TypeId::of::<T>(), vec![])
            .map_or(Ok(()), |_| {
                Err(format!("duplicate component: {}", type_name::<T>()))
            })
    }

    pub fn unregister_component<T: 'static>(&mut self) -> Result<(), String> {
        self.components
            .remove(&TypeId::of::<T>())
            .map_or(Ok(()), |_| {
                Err(format!("unregistered component: {}", type_name::<T>()))
            })
    }

    pub fn add_entity(&mut self) -> EntityHandle {
        let entity = self.next_entity;
        self.next_entity = entity.next();
        self.entities.push(entity);
        EntityHandle {
            world: self,
            entity,
        }
    }

    fn add_component<T: 'static>(&mut self, component: T) -> Result<(), String> {
        let components = self
            .components
            .get_mut(&TypeId::of::<T>())
            .ok_or(format!("unregistered component: {}", type_name::<T>()))?;
        let transmuted = unsafe { mem::transmute::<&mut Vec<u8>, &mut Vec<T>>(components) };
        transmuted.push(component);
        Ok(())
    }

    pub fn add_system<T: 'static>(&mut self, system: fn()) -> Result<(), String> {
        self.systems
            .get_mut(&TypeId::of::<T>())
            .ok_or(format!("unregistered component: {}", type_name::<T>()))?
            .push(system);
        Ok(())
    }

    pub fn get_components<T: 'static>(&self) -> Result<&[T], String> {
        let components = self
            .components
            .get(&TypeId::of::<T>())
            .ok_or(format!("unregistered component: {}", type_name::<T>()))?;

        // Prevents a panic when there's no components
        if components.is_empty() {
            return Ok(&[]);
        }

        let transmuted = unsafe { mem::transmute::<&Vec<u8>, &Vec<T>>(components) };
        Ok(transmuted.as_slice())
    }
}

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
    world: &'a mut World,
    entity: Entity,
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
