pub mod host {
    use aether_core_plugin_pdk::{
        HostReply, Wire,
        api::v0::{
            CachedFileDto, CommandDto, DownloadToCacheParamsDto, HttpRequestDto, HttpResponseDto,
            LogParamsDto, OutputDto,
        },
    };
    use extism_pdk::host_fn;

    #[host_fn]
    extern "ExtismHost" {
        pub fn log(params: Wire<LogParamsDto>) -> HostReply<()>;

        /// A non-zero exit is an answer (`OutputDto::exit_code`), not a failed call: the call
        /// fails only when the command did not run to its end.
        pub fn run_command(command: Wire<CommandDto>) -> HostReply<OutputDto>;

        /// Host-controlled GET (T-0.6, T-0.6.A): `allowed_hosts` and public-address checks are
        /// applied by the host to the first request and to every redirect hop.
        pub fn http_get(request: Wire<HttpRequestDto>) -> HostReply<HttpResponseDto>;

        /// Host-side download into `/cache`, verified against a digest (aether T-1.2): same
        /// network policy as `http_get`, but the body stays out of the plugin's memory.
        pub fn download_to_cache(
            params: Wire<DownloadToCacheParamsDto>,
        ) -> HostReply<CachedFileDto>;
    }
}
