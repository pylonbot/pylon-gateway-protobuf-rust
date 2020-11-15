pub use prost_types;

pub mod discord {
    pub mod v1 {
        pub mod cache {
            include!("stubs/pylon.discord.v1.cache.rs");
        }
        pub mod event {
            include!("stubs/pylon.discord.v1.event.rs");
        }
        pub mod model {
            include!("stubs/pylon.discord.v1.model.rs");
        }
        pub mod rest {
            include!("stubs/pylon.discord.v1.rest.rs");
        }
    }
}

pub mod gateway {
    pub mod v1 {
        pub mod service {
            include!("stubs/pylon.gateway.v1.service.rs");
        }
    }
}
