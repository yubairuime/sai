use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::{self, Value};

pub type EnvRef = Rc<RefCell<RuntimeEnv>>;

#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    pub parent: Option<EnvRef>,
    pub bindings: HashMap<String, Binding>,
}

impl RuntimeEnv {
    pub fn new(parent: Option<EnvRef>) -> EnvRef {
        Rc::new(RefCell::new(Self { parent, bindings: HashMap::new() }))
    }

    pub fn define(env: &EnvRef, name: String, value: Value, mutable: bool) {
        env.borrow_mut().bindings.insert(name, Binding { value, mutable });
    }

    pub fn get(env: &EnvRef, name: &str) -> Option<Binding> {
        let current  = env.borrow();
        if let Some(binding) = current.bindings.get(name) {
            return Some(binding.clone());
        }

        let parent = current.parent.clone();
        drop(current);
        parent.and_then(|parent_env| Self::get(&parent_env, name))
    }

    pub fn assign(env: &EnvRef, name: &str, value: Value) {
        // implicit drop
        {
            let mut current = env.borrow_mut();
            if let Some(binding) = current.bindings.get_mut(name) {
                binding.value = value;
                return;
            }
        }

        let parent = env.borrow().parent.clone().unwrap();
        Self::assign(&parent, name, value)
    }
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub value: Value,
    pub mutable: bool,
}
