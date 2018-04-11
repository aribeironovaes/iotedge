// Copyright (c) Microsoft. All rights reserved.

#![deny(warnings)]

extern crate bytes;
extern crate chrono;
extern crate consistenttime;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate hmac;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

#[macro_use]
extern crate edgelet_utils;

mod crypto;
mod error;
mod identity;
mod module;

use std::rc::Rc;

use futures::{future, future::FutureResult};

pub use error::{Error, ErrorKind};
pub use identity::{Identity, IdentityManager, IdentitySpec};
pub use module::{Module, ModuleRegistry, ModuleRuntime, ModuleRuntimeState, ModuleSpec,
                 ModuleStatus};

pub struct Edgelet<T>
where
    T: ModuleRuntime,
{
    module_runtime: Rc<T>,
}

impl<T> Edgelet<T>
where
    T: ModuleRuntime,
{
    pub fn new(module_runtime: T) -> Edgelet<T> {
        Edgelet {
            module_runtime: Rc::new(module_runtime),
        }
    }

    pub fn start_edge_agent(&self) -> FutureResult<(), Error> {
        // TODO: Implement this in terms of operations on the module runtime
        future::ok(())
    }

    pub fn stop_edge_agent(&self) -> FutureResult<(), Error> {
        // TODO: Implement this in terms of operations on the module runtime
        future::ok(())
    }

    pub fn module_runtime(&self) -> Rc<T> {
        self.module_runtime.clone()
    }
}