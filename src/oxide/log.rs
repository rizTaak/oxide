extern crate log;

#[macro_export(local_inner_macros)]
macro_rules! oxide_error {
    ($($arg:tt)+) => (
        log::error!(target: "oxide", $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! oxide_info {
    ($($arg:tt)+) => (
        log::info!(target: "oxide", $($arg)+);
    )
}
