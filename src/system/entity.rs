use std::collections::VecDeque;

const INDEX_REUSE_THRESHOLD: usize = 2048;
const INDEX_MASK: u32 = (1 << 24) - 1;
const GENERATION_MASK: u32 = (1 << 8) - 1;

pub type Entity = u32;
pub type Generation = u8;

pub struct EntityManager {
    generations: Vec<Generation>,
    pool: VecDeque<Entity>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            generations: Vec::new(),
            pool: VecDeque::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        if self.pool.len() > INDEX_REUSE_THRESHOLD {
            if let Some(index) = self.pool.pop_front() {
                return ((self.generations[index as usize] as u32) << 24) | index;
            }
        }

        self.generations.push(0);
        let index = (self.generations.len() - 1) as u32;
        index
    }

    pub fn alive(&self, entity: Entity) -> bool {
        let generation = ((entity >> 24) & GENERATION_MASK) as u8;
        let index = (entity & INDEX_MASK) as usize;

        self.generations[index] == generation
    }

    pub fn destroy(&mut self, entity: Entity) {
        let index = (entity & INDEX_MASK) as usize;
        self.generations[index] = self.generations[index] + 1;
        self.pool.push_back(index as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut manager = EntityManager::new();

        let first = manager.create();
        let second = manager.create();

        assert_eq!(first, 0);
        assert_eq!(second, 1);
    }

    #[test]
    fn lifecycle() {
        let mut manager = EntityManager::new();

        let entity = manager.create();
        manager.destroy(entity);

        assert!(!manager.alive(entity))
    }

    #[test]
    fn recycle() {
        let mut manager = EntityManager::new();

        for _ in 0..INDEX_REUSE_THRESHOLD + 1 {
            let entity = manager.create();
            manager.destroy(entity);
        }

        let entity = manager.create();

        assert_eq!(entity & INDEX_MASK, 0);
        assert_eq!((entity >> 24) & GENERATION_MASK, 1);
    }
}