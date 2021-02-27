extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=zyre");
    println!("cargo:rustc-link-lib=czmq");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")

        .whitelist_function("zyre_new")
        .whitelist_function("zyre_destroy")
        .whitelist_function("zyre_uuid")
        .whitelist_function("zyre_name")
        .whitelist_function("zyre_set_header")
        .whitelist_function("zyre_set_verbose")
        .whitelist_function("zyre_set_port")
        .whitelist_function("zyre_set_evasive_timeout")
        .whitelist_function("zyre_set_expired_timeout")
        .whitelist_function("zyre_set_interval")
        .whitelist_function("zyre_set_interface")
        .whitelist_function("zyre_set_endpoint")
        .whitelist_function("zyre_gossip_bind")
        .whitelist_function("zyre_gossip_connect")
        .whitelist_function("zyre_start")
        .whitelist_function("zyre_stop")
        .whitelist_function("zyre_join")
        .whitelist_function("zyre_leave")
        .whitelist_function("zyre_recv")
        .whitelist_function("zyre_whisper")
        .whitelist_function("zyre_shout")
        .whitelist_function("zyre_whispers")
        .whitelist_function("zyre_shouts")
        .whitelist_function("zyre_peers")
        .whitelist_function("zyre_peers_by_group")
        .whitelist_function("zyre_own_groups")
        .whitelist_function("zyre_peer_groups")
        .whitelist_function("zyre_peer_address")
        .whitelist_function("zyre_peer_header_value")
        .whitelist_function("zyre_socket")
        .whitelist_function("zyre_print")
        .whitelist_function("zyre_version")

        .whitelist_function("zyre_event_new")
        .whitelist_function("zyre_event_destroy")
        .whitelist_function("zyre_event_type")
        .whitelist_function("zyre_event_peer_uuid")
        .whitelist_function("zyre_event_peer_name")
        .whitelist_function("zyre_event_peer_addr")
        .whitelist_function("zyre_event_headers")
        .whitelist_function("zyre_event_header")
        .whitelist_function("zyre_event_group")
        .whitelist_function("zyre_event_msg")
        .whitelist_function("zyre_event_get_msg")
        .whitelist_function("zyre_event_print")
        .whitelist_function("zyre_event_test")

        .whitelist_function("zmsg_new")
        .whitelist_function("zmsg_destroy")
        .whitelist_function("zmsg_size")
        .whitelist_function("zmsg_pushstr")
        .whitelist_function("zmsg_popstr")

        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source = bindings
        .to_string()
        .replace("pub _:", "pub __RENAMED__:");

    let mut f = File::create(out_path.join("bindings.rs"))
        .expect("Could not open bindings file.");
    f.write_all(source.as_bytes())
        .expect("Could not write bindings.");
}
