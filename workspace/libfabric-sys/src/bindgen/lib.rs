// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.



extern crate libc;


use ::core::fmt::Debug;
use ::core::fmt::Formatter;
use ::core::fmt::Result;
use ::core::marker::Copy;
use ::core::marker::PhantomData;
use ::core::mem::transmute;
use ::core::mem::zeroed;
use ::core::option::Option;
use ::core::slice::from_raw_parts;
use ::core::slice::from_raw_parts_mut;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_ulonglong;
use ::libc::c_void;

use ::libc::iovec;
use ::libc::pthread_cond_t;
use ::libc::pthread_mutex_t;

#[link(name = "libfabric", kind = "static-nobundle")]
extern "C"
{
}

include!("bindgen/constants.rs");
include!("bindgen/types.rs");
include!("bindgen/enums.rs");
include!("bindgen/structs.rs");
include!("bindgen/unions.rs");
include!("bindgen/statics.rs");
include!("bindgen/functions.rs");
include!("bindgen/opaques.rs");
