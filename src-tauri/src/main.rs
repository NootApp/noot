// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    noot_lib::run();

    noot_lib::rpc::shutdown_rpc();
}
