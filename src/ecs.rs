use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Entity<P> {
    index: usize, 
    generation: usize,
    phantom: PhantomData<P>,
}

pub struct EntityManager<P> {
    free_entities: Vec<Option<usize>>,
    generations: Vec<usize>,
    first_free: Option<usize>,
    phantom: PhantomData<P>,
}

pub struct Components<T, P> {
    entities: Vec<Entity<P>>,
    values: Vec<T>,
    phantom: PhantomData<P>,
}

impl<P> Entity<P> where P: Copy + Clone {
    #[allow(dead_code)]
    pub fn new(index: usize, generation: usize) -> Self {
        Self {
            index: index,
            generation: generation,
            phantom: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Entity::new(0, 0)
    }
}

impl<P> std::cmp::PartialEq for Entity<P> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

const STARTING_GENERATION: usize = 1;

impl<T> EntityManager<T> where T: Copy + Clone {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            free_entities: vec![None],
            generations: vec![STARTING_GENERATION],
            first_free: Some(0),
            phantom: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn is_alive(&self, e: Entity<T>) -> bool {
        e.generation == self.generations[e.index]
    }

    #[allow(dead_code)]
    pub fn allocate(&mut self) -> Entity<T> {
        if let Some(index) = self.first_free {
            self.first_free = self.free_entities[index];
            Entity::new(index, self.generations[index])
        }
        else {
            self.first_free = None;
            let index = self.free_entities.len();
            self.free_entities.push(None);
            self.generations.push(STARTING_GENERATION);
            Entity::new(index, STARTING_GENERATION)
        }
    }

    #[allow(dead_code)]
    pub fn deallocate(&mut self, e: Entity<T>) {
        if self.is_alive(e) {
            let index = e.index;
            self.free_entities[index] = self.first_free;
            self.first_free = Some(index);
            self.generations[index] += 1;
        }
    }
}

impl<T, P> Components<T, P> where T: Default, P: Copy + Clone {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: vec![],
            entities: vec![],
            phantom: PhantomData,
        }
    }

    fn resize(&mut self, new_len: usize) {
        let len = self.entities.len();
        if len < new_len {
            for _ in 0..(new_len - len) {
                self.entities.push(Entity::<P>::new_empty());
                self.values.push(Default::default());
            }
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, e: Entity<P>, v: T) {
        self.resize(e.index + 1);
        self.entities[e.index] = e;
        self.values[e.index] = v;
    }

    #[allow(dead_code)]
    pub fn add(&mut self, em: &mut EntityManager<P>, v: T) -> Entity<P> {
        let e = em.allocate();
        self.set(e, v);
        e
    }

    #[allow(dead_code)]
    pub fn get(&self, e: Entity<P>) -> &T {
        &self.values[e.index]
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, e: Entity<P>) -> &mut T {
        &mut self.values[e.index]
    }

    #[allow(dead_code)]
    pub fn iter(&self)
        -> std::iter::Zip<std::slice::Iter<'_, Entity<P>>, std::slice::Iter<'_, T>>
    {
        self.entities.iter().zip(self.values.iter())
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self)
        -> std::iter::Zip<std::slice::Iter<'_, Entity<P>>, std::slice::IterMut<'_, T>>
    {
        self.entities.iter().zip(self.values.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    struct TestComp {}

    #[test]
    fn test_em() {
        let mut em = EntityManager::<TestComp>::new();
        let e1 = em.allocate();
        assert_eq!(e1.index, 0);
        assert_eq!(e1.generation, 1);
        assert!(em.is_alive(e1));

        let e2 = em.allocate();
        assert_eq!(e2.index, 1);
        assert_eq!(e2.generation, 1);
        assert!(em.is_alive(e2));

        let e3 = em.allocate();
        assert_eq!(e3.index, 2);
        assert_eq!(e3.generation, 1);
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
        assert_eq!(e1.generation, 2);
        assert!(em.is_alive(e1));

        assert_eq!(em.first_free, Some(1));
    }

    #[test]
    fn test_component() {
        let mut components = Components::<usize, TestComp>::new();
        let mut em = EntityManager::<TestComp>::new();
        let e1 = em.allocate();
        components.set(e1, 100);
        assert_eq!(*components.get(e1), 100);
    }

    #[test]
    fn test_component_iter() {
        let mut components = Components::<usize, TestComp>::new();
        let mut em = EntityManager::<TestComp>::new();
        components.add(&mut em, 11);
        components.add(&mut em, 22);
        components.add(&mut em, 33);

        let mut it = components.iter();

        let (e, v) = it.next().unwrap();
        assert_eq!(*v, 11);
        assert_eq!(e.index, 0);
        assert_eq!(e.generation, 1);

        let (e, v) = it.next().unwrap();
        assert_eq!(*v, 22);
        assert_eq!(e.index, 1);
        assert_eq!(e.generation, 1);

        let (e, v) = it.next().unwrap();
        assert_eq!(*v, 33);
        assert_eq!(e.index, 2);
        assert_eq!(e.generation, 1);

        assert_eq!(it.next(), None);
    }
}
