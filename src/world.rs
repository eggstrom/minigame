use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    collections::HashMap,
    mem,
    rc::Rc,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::{DrawData, GameRequest, SharedState};

pub struct WorldModule {
    ticks_per_second: u16,
    tick_instant: Instant,
    world: Rc<UnsafeCell<World>>,
}

impl<'a> WorldModule {
    pub fn new(ticks_per_second: u16) -> Self {
        WorldModule {
            ticks_per_second,
            tick_instant: Instant::now(),
            world: Rc::new(UnsafeCell::new(World::new())),
        }
    }

    pub fn start(&mut self, state: Arc<SharedState>, init: fn(&mut GameHandle)) {
        init(&mut self.game_handle(state));
    }

    pub fn update(&mut self, state: Arc<SharedState>) -> Result<(), String> {
        // let instant = Instant::now();

        let game = self.game_handle(Arc::clone(&state));
        let world = unsafe { &mut *self.world.get() };
        world.update(game);

        // println!("World update took {}us", instant.elapsed().as_micros());

        state.set_draw_data(mem::take(&mut world.draw_data))?;
        self.await_next_tick();
        Ok(())
    }

    fn await_next_tick(&mut self) {
        let tick_duration = Duration::from_secs(1) / self.ticks_per_second as u32;
        thread::sleep(tick_duration.saturating_sub(self.tick_instant.elapsed()));
        self.tick_instant = Instant::now();
    }

    fn game_handle(&'a self, state: Arc<SharedState>) -> GameHandle {
        GameHandle {
            world: Rc::clone(&self.world),
            state,
        }
    }
}

pub struct World {
    next_entity: Entity,
    entities: Vec<Entity>,
    storage: HashMap<TypeId, Box<dyn WorldStorageTrait>>,
    draw_data: Vec<DrawData>,
}

impl<'a> World {
    pub fn new() -> Self {
        World {
            next_entity: Entity::default(),
            entities: vec![],
            storage: HashMap::new(),
            draw_data: vec![],
        }
    }

    pub fn update(&mut self, mut game: GameHandle) {
        for storage in self.storage.values_mut() {
            storage.update(&mut game);
        }
    }

    fn storage<C: 'static>(&mut self) -> &mut WorldStorage<C> {
        let storage = self
            .storage
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(WorldStorage::<C>::new()));
        let storage: &mut Box<dyn Any> = unsafe { mem::transmute(storage) };
        storage.downcast_mut().unwrap()
    }

    pub fn add_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity = entity.next();
        self.entities.push(entity);
        entity
    }

    pub fn add_system<C: 'static>(&mut self, system: fn(&mut GameHandle, Entity, &mut C)) {
        self.storage::<C>().add_system(system);
    }

    pub fn add_component<C: 'static>(&mut self, entity: Entity, component: C) {
        self.storage::<C>().add_component(entity, component);
    }
}

pub struct WorldStorage<C> {
    systems: Vec<fn(&mut GameHandle, Entity, &mut C)>,
    components: Vec<C>,
    entities: Vec<Entity>,
}

impl<C> WorldStorage<C> {
    pub fn new() -> Self {
        WorldStorage {
            systems: vec![],
            components: vec![],
            entities: vec![],
        }
    }

    pub fn add_system(&mut self, system: fn(&mut GameHandle, Entity, &mut C)) {
        self.systems.push(system);
    }

    pub fn clear_systems(&mut self) {
        self.systems.clear();
    }

    pub fn add_component(&mut self, entity: Entity, component: C) {
        self.entities.push(entity);
        self.components.push(component);
    }

    pub fn remove_component(&mut self, entity: Entity) {
        if let Some(index) = self.entities.iter().position(|&e| e == entity) {
            self.entities.swap_remove(index);
            self.components.swap_remove(index);
        }
    }

    pub fn clear_components(&mut self) {
        self.entities.clear();
        self.components.clear();
    }
}

pub trait WorldStorageTrait: Any {
    fn update(&mut self, game: &mut GameHandle);
}

impl<C: 'static> WorldStorageTrait for WorldStorage<C> {
    fn update(&mut self, game: &mut GameHandle) {
        let mut systems = mem::take(&mut self.systems);
        let mut components = mem::take(&mut self.components);
        let mut entities = mem::take(&mut self.entities);

        for system in systems.iter() {
            for (entity, component) in entities.iter().zip(components.iter_mut()) {
                system(game, *entity, component);
            }
        }

        self.systems = mem::take(&mut systems);
        self.components = mem::take(&mut components);
        self.entities = mem::take(&mut entities);
    }
}

pub struct GameHandle {
    world: Rc<UnsafeCell<World>>,
    state: Arc<SharedState>,
}

impl GameHandle {
    pub fn stop(&self) {
        self.state.stop();
    }

    pub fn send<R: GameRequest>(&self, request: R) -> Result<(), String> {
        request.send(&self.state)?;
        Ok(())
    }

    pub fn draw(&mut self, data: DrawData) {
        let world = unsafe { &mut *self.world.get() };
        world.draw_data.push(data);
    }

    pub fn add_system<C: 'static>(&mut self, system: fn(&mut GameHandle, Entity, &mut C)) {
        let world = unsafe { &mut *self.world.get() };
        world.add_system(system);
    }

    pub fn add_entity(&mut self) -> EntityHandle {
        let world = unsafe { &mut *self.world.get() };
        let entity = world.add_entity();
        EntityHandle {
            world: Rc::clone(&self.world),
            entity,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

pub struct EntityHandle {
    world: Rc<UnsafeCell<World>>,
    entity: Entity,
}

impl EntityHandle {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn add_component<C: 'static>(&mut self, component: C) -> &mut Self {
        let world = unsafe { &mut *self.world.get() };
        world.add_component(self.entity, component);
        self
    }
}
