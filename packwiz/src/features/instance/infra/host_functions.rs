pub mod host {
    use std::collections::HashMap;

    use extism_pdk::{host_fn, Msgpack};

    use crate::features::{
        host::HostResult,
        instance::{ContentFile, NewInstance},
    };

    #[host_fn]
    extern "ExtismHost" {
        pub fn instance_get_dir(instance_id: String) -> HostResult<String>;
        pub fn instance_plugin_get_dir(instance_id: String) -> HostResult<String>;

        pub fn instance_create(new_instance: NewInstance) -> HostResult<String>;

        pub fn list_content(instance_id: String) -> HostResult<HashMap<String, ContentFile>>;
        pub fn enable_contents(
            instance_id: String,
            content_paths: Msgpack<Vec<String>>,
        ) -> HostResult<()>;
        pub fn disable_contents(
            instance_id: String,
            content_paths: Msgpack<Vec<String>>,
        ) -> HostResult<()>;
    }
}
