/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// The idea hear is the same as contexts in golang.
// a veriable that manages lifetimes
use std::time::{Instant, Duration};
use std::sync::{
    Arc,
    Weak,
    RwLock,
    atomic::{
        Ordering,
        AtomicBool
    }
};
use super::Error::{self, ContextError};

struct InternalContext {
    created: RwLock<Instant>,
    ttl: Option<Duration>,
    parent: Option<Weak<InternalContext>>,
    alive: Arc<AtomicBool>,
}

impl InternalContext {
    fn is_alive(&self) -> bool {
        if let Some(ptr) = &self.parent {
            if let Some(p) = Weak::upgrade(&ptr) {
                if !Context(p).is_alive() {
                    return false;
                } 
            } else {
                return false;
            }
        }

        // Check if there is a ttl
        if let Some(ttl) = &self.ttl {
            if self.created.read().unwrap().elapsed() >= *ttl {
                return false;
            }
        }

        return self.alive.load(Ordering::SeqCst);
    }
}

pub struct Context(Arc<InternalContext>);

impl Context {
    pub fn new() -> Self {
        Context(Arc::new(InternalContext {
            created: RwLock::new(Instant::now()),
            ttl: None,
            parent: None,
            alive: Arc::new(AtomicBool::new(true)),
        }))
    }

    pub fn cancel(&mut self) {
        self.0.alive.store(false, Ordering::SeqCst);
    }

    pub fn new_with_duration(ttl: Duration) -> Self {
        Context(Arc::new(InternalContext {
            created: RwLock::new(Instant::now()),
            ttl: Some(ttl),
            parent: None,
            alive: Arc::new(AtomicBool::new(true)),
        }))
    }

    fn fields(&self) -> &Arc<InternalContext> {
        return &self.0;
    }


    pub fn reset(&self) -> Result<(), Error>{
        // validate that Parent is still alive
        if let Some(ptr) = &self.fields().parent {
            if let Some(p) = Weak::upgrade(&ptr) {
                if !p.is_alive() {
                    return Err(ContextError(String::from("Parent Context Dead")));
                }
            }
        }

        self.fields().alive.load(Ordering::SeqCst);
        let mut r = self.fields().created.write().unwrap();
        *r = Instant::now();
        Ok(())
    }

    pub fn child(&self) -> Context {
        Context(Arc::new(InternalContext{
            created: RwLock::new(Instant::now()),
            ttl: None,
            parent: Some(Arc::downgrade(&self.0)),
            alive: Arc::new(AtomicBool::new(true)),
        }))
    }

    pub fn with_duration(&self, ttl: Duration) -> Context {
        Context(Arc::new(InternalContext {
            created: RwLock::new(Instant::now()),
            ttl: Some(ttl),
            parent: Some(Arc::downgrade(&self.0)),
            alive: Arc::new(AtomicBool::new(true)),
        }))
    }


    pub fn is_alive(&self) -> bool {
        return self.0.is_alive();
    }
}


#[cfg(test)]
mod test {
    use crate::engine::Context;
    use std::{
        thread::{
        spawn,
        sleep
        },
        time::Duration
    };

    #[test]
    fn threaded_lifetime_test() {
        let r: Context = Context::new();
        let c: Context = r.child();
        let h1 =spawn(move || {
            while c.is_alive() {
                sleep(Duration::from_nanos(50));
            }
            assert_eq!(c.is_alive(), false);
        });
        let h2 = spawn( move || {
            sleep(Duration::from_nanos(200));
            drop(r);
        });

        _ = h1.join();
        _ = h2.join();

    }

    #[test]
    fn cancel_parent_test() {
        let mut r = Context::new();
        let c: Context = r.child();

        assert_eq!(c.is_alive(), true);
        r.cancel();
        assert_eq!(r.is_alive(), false);
        assert_eq!(c.is_alive(), false);
    }

    #[test]
    fn cancel_grandparent_test() {
        let mut r = Context::new();
        let c = r.child();
        let gc = c.child();

        assert_eq!(gc.is_alive(), true);
        r.cancel();
        assert_eq!(r.is_alive(), false);
        assert_eq!(c.is_alive(), false);
        assert_eq!(gc.is_alive(), false);
    }

    #[test]
    fn duration_test() {
        let r = Context::new_with_duration(Duration::from_millis(20));
        assert_eq!(r.is_alive(), true);
        sleep(Duration::from_millis(25));
        assert_eq!(r.is_alive(), false);
    }

    #[test]
    fn duration_child_test() {
        let r = Context::new_with_duration(Duration::from_millis(20));
        let c = r.child();
        assert_eq!(c.is_alive(), true);
        sleep(Duration::from_millis(25));
        assert_eq!(c.is_alive(), false);

    }
}