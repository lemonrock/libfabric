// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(associated_consts)]


extern crate libc;
extern crate libfabric_sys;
extern crate rust_extra;


use ::libc::c_ulonglong;
use ::libfabric_sys::*;
use ::rust_extra::unlikely;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;


include!("panic_on_error.rs");


include!("ControlCommand.rs");
include!("EndpointProtocol.rs");
include!("EventQueue.rs");
include!("Fabric.rs");
include!("Provider.rs");
include!("TransportAddressFormat.rs");
include!("Version.rs");
