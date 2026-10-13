pub mod host {
    use aether_core_plugin_api::v0::{
        CachedFileDto, CommandDto, DownloadToCacheParamsDto, HttpRequestDto, HttpResponseDto,
        OutputDto,
    };
    use extism_pdk::{Msgpack, host_fn};

    use crate::features::host::HostResult;

    #[host_fn]
    extern "ExtismHost" {
        pub fn log(level: u32, msg: String);
        pub fn get_id() -> String;

        pub fn run_command(command: Msgpack<CommandDto>) -> HostResult<OutputDto>;

        /// Host-controlled GET (T-0.6, T-0.6.A): `allowed_hosts` and public-address checks are
        /// applied by the host to the first request and to every redirect hop.
        pub fn http_get(request: Msgpack<HttpRequestDto>) -> HostResult<HttpResponseDto>;

        /// Host-side download into `/cache`, verified against a digest (aether T-1.2): same
        /// network policy as `http_get`, but the body stays out of the plugin's memory.
        pub fn download_to_cache(
            params: Msgpack<DownloadToCacheParamsDto>,
        ) -> HostResult<CachedFileDto>;
    }
}
