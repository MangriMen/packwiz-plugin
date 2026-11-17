pub mod host {
    use aether_core_plugin_pdk::{
        HostReply, Wire,
        api::v0::{
            ChangeContentsParamsDto, InstanceCreatedDto, InstanceDirDto, InstanceIdParamsDto,
            ListContentResultDto, NewInstanceDto, WriteInstanceFileParamsDto,
        },
    };
    use extism_pdk::host_fn;

    #[host_fn]
    extern "ExtismHost" {
        pub fn instance_get_dir(params: Wire<InstanceIdParamsDto>) -> HostReply<InstanceDirDto>;
        pub fn instance_plugin_get_dir(
            params: Wire<InstanceIdParamsDto>,
        ) -> HostReply<InstanceDirDto>;

        pub fn instance_create(new_instance: Wire<NewInstanceDto>)
        -> HostReply<InstanceCreatedDto>;

        pub fn list_content(params: Wire<InstanceIdParamsDto>) -> HostReply<ListContentResultDto>;
        pub fn enable_contents(params: Wire<ChangeContentsParamsDto>) -> HostReply<()>;
        pub fn disable_contents(params: Wire<ChangeContentsParamsDto>) -> HostReply<()>;

        /// The only way to put a file into an instance directory (T-1.3): the plugin names the
        /// instance by id and the file by a path relative to the instance root, the host decides
        /// where that lands on disk and enforces the segment rules and the 64 MiB cap.
        pub fn write_instance_file(params: Wire<WriteInstanceFileParamsDto>) -> HostReply<()>;
    }
}
