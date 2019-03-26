use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Entity<T> {
    index: usize, 
    generation: usize,
    phantom: PhantomData<T>,
}

pub struct EntityManager<T> {
    free_entities: Vec<Option<usize>>,
    generations: Vec<usize>,
    first_free: Option<usize>,
    phantom: PhantomData<T>,
}

pub struct Component<V, T> {
    values: Vec<V>,
    phantom: PhantomData<T>,
}

impl<T> Entity<T> where T: Copy + Clone {
    pub fn new(index: usize, generation: usize) -> Self {
        Self {
            index: index,
            generation: generation,
            phantom: PhantomData,
        }
    }
}

impl<T> EntityManager<T> where T: Copy + Clone {
    pub fn new() -> Self {
        Self {
            free_entities: vec![None],
            generations: vec![0],
            first_free: Some(0),
            phantom: PhantomData,
        }
    }

    pub fn is_alive(&self, e: Entity<T>) -> bool {
        e.generation == self.generations[e.index]
    }

    pub fn allocate(&mut self) -> Entity<T> {
        if let Some(index) = self.first_free {
            self.first_free = self.free_entities[index];
            Entity::new(index, self.generations[index])
        }
        else {
            self.first_free = None;
            let index = self.free_entities.len();
            self.free_entities.push(None);
            self.generations.push(0);
            Entity::new(index, 0)
        }
    }

    pub fn deallocate(&mut self, e: Entity<T>) {
        if self.is_alive(e) {
            let index = e.index;
            self.free_entities[index] = self.first_free;
            self.first_free = Some(index);
            self.generations[index] += 1;
        }
    }
}

impl<V, T> Component<V, T> where T: Copy + Clone, V: Default {
    pub fn new() -> Self {
        Self {
            values: vec![],
            phantom: PhantomData,
        }
    }

    fn resize(&mut self, new_len: usize) {
        let len = self.values.len();
        if len < new_len {
            for _ in 0..(new_len - len) {
                self.values.push(Default::default());
            }
        }
    }

    pub fn set(&mut self, e: Entity<T>, v: V) {
        self.resize(e.index + 1);
        self.values[e.index] = v;
    }

    pub fn get(&self, e: Entity<T>) -> &V {
        &self.values[e.index]
    }

    pub fn get_mut(&mut self, e: Entity<T>) -> &mut V {
        &mut self.values[e.index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_em() {
        type T = usize;
        let mut em = EntityManager::<T>::new();
        let e1 = em.allocate();
        assert_eq!(e1.index, 0);
        assert_eq!(e1.generation, 0);
        assert!(em.is_alive(e1));

        let e2 = em.allocate();
        assert_eq!(e2.index, 1);
        assert_eq!(e2.generation, 0);
        assert!(em.is_alive(e2));

        let e3 = em.allocate();
        assert_eq!(e3.index, 2);
        assert_eq!(e3.generation, 0);
        assert!(em.is_alive(e3));

        assert_eq!(em.free_entities.len(), 3);

        em.deallocate(e2);

        assert!(em.is_alive(e1));
        assert!(!em.is_alive(e2));
        assert!(em.is_alive(e3));

        em.deallocate(e1);

        assert!(!em.is_alive(e1));
        assert!(!em.is_alive(e2));
        assert!(em.is_alive(e3));

        assert_eq!(em.free_entities.len(), 3);

        assert_eq!(em.first_free, Some(0));

        let e1 = em.allocate();
        assert_eq!(e1.index, 0);
        assert_eq!(e1.generation, 1);
        assert!(em.is_alive(e1));

        assert_eq!(em.first_free, Some(1));
    }

    #[test]
    fn test_component() {
    }
}
