extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=zyre");
    println!("cargo:rustc-link-lib=czmq");

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.h")

        .whitelisted_function("zyre_new")
        .whitelisted_function("zyre_destroy")
        .whitelisted_function("zyre_uuid")
        .whitelisted_function("zyre_name")
        .whitelisted_function("zyre_set_header")
        .whitelisted_function("zyre_set_verbose")
        .whitelisted_function("zyre_set_port")
        .whitelisted_function("zyre_set_evasive_timeout")
        .whitelisted_function("zyre_set_expired_timeout")
        .whitelisted_function("zyre_set_interval")
        .whitelisted_function("zyre_set_interface")
        .whitelisted_function("zyre_set_endpoint")
        .whitelisted_function("zyre_gossip_bind")
        .whitelisted_function("zyre_gossip_connect")
        .whitelisted_function("zyre_start")
        .whitelisted_function("zyre_stop")
        .whitelisted_function("zyre_join")
        .whitelisted_function("zyre_leave")
        .whitelisted_function("zyre_recv")
        .whitelisted_function("zyre_whisper")
        .whitelisted_function("zyre_shout")
        .whitelisted_function("zyre_whispers")
        .whitelisted_function("zyre_shouts")
        .whitelisted_function("zyre_peers")
        .whitelisted_function("zyre_peers_by_group")
        .whitelisted_function("zyre_own_groups")
        .whitelisted_function("zyre_peer_groups")
        .whitelisted_function("zyre_peer_address")
        .whitelisted_function("zyre_peer_header_value")
        .whitelisted_function("zyre_socket")
        .whitelisted_function("zyre_print")
        .whitelisted_function("zyre_version")

        .whitelisted_function("zyre_event_new")
        .whitelisted_function("zyre_event_destroy")
        .whitelisted_function("zyre_event_type")
        .whitelisted_function("zyre_event_peer_uuid")
        .whitelisted_function("zyre_event_peer_name")
        .whitelisted_function("zyre_event_peer_addr")
        .whitelisted_function("zyre_event_headers")
        .whitelisted_function("zyre_event_header")
        .whitelisted_function("zyre_event_group")
        .whitelisted_function("zyre_event_msg")
        .whitelisted_function("zyre_event_get_msg")
        .whitelisted_function("zyre_event_print")
        .whitelisted_function("zyre_event_test")

        .whitelisted_function("zmsg_new")
        .whitelisted_function("zmsg_destroy")
        .whitelisted_function("zmsg_size")
        .whitelisted_function("zmsg_pushstr")
        .whitelisted_function("zmsg_popstr")

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
