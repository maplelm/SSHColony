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
use crate::engine::error::{Error, ErrorKind};
use std::sync::{
    Arc, RwLock, Weak,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct InternalContext {
    created: Instant,
    ttl: Option<Duration>,
    parent: Weak<InternalContext>,
    alive: AtomicBool,
}

impl InternalContext {
    fn is_alive(&self) -> bool {
        if let Some(p) = Weak::upgrade(&self.parent) {
            if !Context(p).is_alive() {
                return false;
            }
        }
        // Check if there is a ttl
        if let Some(ttl) = self.ttl.as_ref() {
            if self.created.elapsed() >= *ttl {
                return false;
            }
        }

        return self.alive.load(Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct Context(Arc<InternalContext>);

impl Context {
    pub fn new() -> Self {
        Context(Arc::new(InternalContext {
            created: Instant::now(),
            ttl: None,
            parent: Weak::new(),
            alive: AtomicBool::new(true),
        }))
    }

    pub fn cancel(&self) {
        self.0.alive.store(false, Ordering::SeqCst)
    }

    pub fn new_with_duration(ttl: Duration) -> Self {
        Context(Arc::new(InternalContext {
            created: Instant::now(),
            ttl: Some(ttl),
            parent: Weak::new(),
            alive: AtomicBool::new(true),
        }))
    }

    pub fn child(&self) -> Context {
        Context(Arc::new(InternalContext {
            created: Instant::now(),
            ttl: None,
            parent: Arc::downgrade(&self.0),
            alive: AtomicBool::new(true),
        }))
    }

    pub fn with_duration(&self, ttl: Duration) -> Context {
        Context(Arc::new(InternalContext {
            created: Instant::now(),
            ttl: Some(ttl),
            parent: Arc::downgrade(&self.0),
            alive: AtomicBool::new(true),
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
        thread::{sleep, spawn},
        time::Duration,
    };

    #[test]
    fn threaded_lifetime_test() {
        let r: Context = Context::new();
        let c: Context = r.child();
        let h1 = spawn(move || {
            while c.is_alive() {
                sleep(Duration::from_nanos(50));
            }
            assert_eq!(c.is_alive(), false);
        });
        let h2 = spawn(move || {
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
