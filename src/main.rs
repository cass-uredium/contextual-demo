#![feature(macro_metavar_expr_concat)]

use application::App;

pub mod accessibility;
mod application;
mod controller;
mod event;

#[cfg(not(target_os = "macos"))]
compile_error!("crate only supported on macOS");

fn main() -> anyhow::Result<()> {
    App::default().run()
}
