use super::backend::{Backend, BackendConfig};
use super::target::Target;
use crate::generator::Generator;
use crate::rustgenerator::RustGenerator;
use crate::cgenerator::CGenerator;
use std::io::Write;

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub target: Target,
    pub backend: Backend,
    pub backend_config: BackendConfig,
}

impl Config {
    pub fn parse(target_opt: &str, backend_opt: &str, zero_native_pointers: bool) -> Self {
        let mut target = Target::from(target_opt);
        let backend = Backend::from(backend_opt);
        if zero_native_pointers {
            target = Target::Generic;
        }
        let backend_config = BackendConfig {
            zero_native_pointers,
        };
        Self {
            target,
            backend,
            backend_config,
        }
    }

    pub fn generator<W: Write>(&self) -> Box<dyn Generator<W>> {
        match self.backend {
            Backend::C => Box::new(CGenerator {
                target: self.target,
                backend_config: self.backend_config,
            }),

            Backend::Rust => Box::new(RustGenerator {
                target: self.target,
                backend_config: self.backend_config,
            })
        }
    }
}


