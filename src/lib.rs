#![feature(test)]

extern crate test;

use std::sync::{Arc};
use std::rc::Rc;
use std::cell::RefCell;
use test::Bencher;

#[derive(Debug)]
pub struct World {
    pub sex: Vec<bool>,
    pub alive: Vec<bool>,
    pub childs: Vec<Vec<usize>>,
    pub parents: Vec<(Option<usize>, Option<usize>)>,
    pub alive_count: usize,
    pub total_count: usize,
}

impl World {
    pub fn new() -> Self {
        World {
            sex: vec![],
            alive: vec![],
            childs: vec![],
            parents: vec![],
            alive_count: 0,
            total_count: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        World {
            sex: Vec::with_capacity(capacity),
            alive: Vec::with_capacity(capacity),
            childs: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
            alive_count: 0,
            total_count: 0,
        }
    }

    pub fn create_person(&mut self, parent_male: Option<usize>, parent_female: Option<usize>) -> usize {
        let sex = false; //todo: random
        self.sex.push(sex);
        self.alive.push(true);
        self.childs.push(vec![]);
        self.parents.push((parent_male, parent_female));

        let id = self.total_count;

        self.alive_count = self.alive_count + 1;
        self.total_count = self.total_count + 1;

        if let Some(pm) = parent_male {
            self.childs.get_mut(pm).expect(&format!("Invalid parent id {} passed to create_person.", pm)).push(id);
        }

        if let Some(pf) = parent_female {
            self.childs.get_mut(pf).expect(&format!("Invalid parent id {} passed to create_person.", pf)).push(id);
        }

        id
    }

    pub fn kill(&mut self, id: usize) -> bool {
        if let Some(e) = self.alive.get_mut(id) {
            if *e {
                *e = false;
                self.alive_count = self.alive_count - 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

// How many persons are alive?
// How many persons is there in total?
// Who are the parents of this person?
pub struct OOPPerson {
    pub alive: bool,
    pub sex: bool,
    pub childrens: Vec<Rc<RefCell<OOPPerson>>>,
}

impl OOPPerson {
    pub fn new() -> Self {
        OOPPerson {
            alive: true,
            sex: false, // TODO: random
            childrens: vec![],
        }
    }
}

/*pub trait PersonView {
    fn is_alive(&self) -> bool;
    fn is_male(&self) -> bool;
    fn is_female(&self) -> bool {
        !self.is_male()
    }
    fn childs(&self) -> Vec<Box<PersonView + 'static>>;
    fn parents(&self) ->(Option<Box<PersonView + 'static>>, Option<Box<PersonView + 'static>>);
}

pub struct PersonViewImpl {
    is_alive: bool,
    sex: bool,
    childs: Vec<usize>,
    parents: (Option<usize>, Option<usize>),
}

impl PersonView for PersonViewImpl {
    fn is_alive(&self) -> bool;
    fn is_male(&self) -> bool;
    fn is_female(&self) -> bool {
        !self.is_male()
    }
    fn childs(&self) -> Vec<Box<PersonView + 'static>>;
    fn parents(&self) ->(Option<Box<PersonView + 'static>>, Option<Box<PersonView + 'static>>) {

    }
}*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_kill_person() {
        let mut world = World::new();
        assert_eq!(world.alive_count, 0);
        assert_eq!(world.total_count, 0);

        let id = world.create_person(None, None);

        assert_eq!(world.alive_count, 1);
        assert_eq!(world.total_count, 1);

        let killed = world.kill(id);

        assert!(killed);
        assert_eq!(world.alive_count, 0);
        assert_eq!(world.total_count, 1);
    }

    #[test]
    fn complex_tree() {
        // 0 1
        // 2..12 | 13..23
        let mut world = World::with_capacity(111112);

        world.create_person(None, None);
        world.create_person(None, None);

        for i in 0usize..5 {
            // i = depth

            for j in 0usize..10usize.pow(i as u32) {
                // element at this depth
                let parent_m = i * j;
                let parent_f = (i * j) + 1;
                for _ in 0usize..10 {
                    // create child
                    world.create_person(Some(parent_m), Some(parent_f));
                }
            }
        }
        assert_eq!(world.total_count, 111112);
    }

    #[test]
    fn complex_tree_oop() {

        let root1 = Rc::new(RefCell::new(OOPPerson::new()));
        let root2 = Rc::new(RefCell::new(OOPPerson::new()));

        // two elements
        // create 10 childs
        // foreach child, group current + next (last + first at the end)
        //   repeat

        fn create_childs(count: i32,depth: i32, from1: Rc<RefCell<OOPPerson>>, from2: Rc<RefCell<OOPPerson>>) -> i32 {
            if depth <= 0 {
                return count;
            }

            let mut new_count = count;

            for _ in 0..10{
                let child = Rc::new(RefCell::new(OOPPerson::new()));
                from1.borrow_mut().childrens.push(child.clone());
                from2.borrow_mut().childrens.push(child.clone());

                new_count += 1;
            }
            // recurse
            for i in 0..10 {
                let j = if i == 9 {
                    0
                } else {
                    i + 1
                };
                new_count += create_childs(count, depth - 1, from1.borrow_mut().childrens.get(i).unwrap().clone(), from2.borrow_mut().childrens.get(j).unwrap().clone());
            }
            new_count
        }

        let count = create_childs(0, 5, root1, root2);

        assert_eq!(count + 2, 111112);// Adding the root elements
    }

    #[bench]
    fn bench_data_oriented(b: &mut Bencher) {
        b.iter(|| {
            complex_tree();
        })
    }

    #[bench]
    fn bench_data_oop(b: &mut Bencher) {
        b.iter(|| {
            complex_tree_oop();
        })
    }

}
