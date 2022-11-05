#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::ToString;
use casper_types::{Key,CLType,Parameter,EntryPoint,EntryPoints,EntryPointType,EntryPointAccess};
use casper_contract::{
    contract_api::{runtime,storage},
    unwrap_or_revert::UnwrapOrRevert,
};

#[no_mangle]
pub extern "C" fn update_msg() {
    let value: String = runtime::get_named_arg("message");
    let uref=runtime::get_key("message").unwrap_or_revert().into_uref().unwrap_or_revert();
    storage::write(uref,String::from(value));
}

#[no_mangle]
pub extern "C" fn call() {
    let value:String = runtime::get_named_arg("message");
    let value_ref = storage::new_uref(value);
    let mut named_keys:BTreeMap<String,Key> = BTreeMap::new();
    named_keys.insert(String::from("message"),value_ref.into());
    let mut vec = Vec::new();
    vec.push(Parameter::new("message",CLType::String));
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "update_msg",
        vec,
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("hello_world_package_name".to_string()),
        Some("hello_world_access_uref".to_string()),
    );

    runtime::put_key("hello_world_contract",stored_contract_hash.into());
}