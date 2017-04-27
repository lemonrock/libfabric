// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn fi_dupinfo(info: *const fi_info) -> *mut fi_info;
	pub fn fi_freeinfo(info: *mut fi_info);
	pub fn fi_getinfo(version: u32, node: *const c_char, service: *const c_char, flags: u64, hints: *mut fi_info, info: *mut *mut fi_info) -> c_int;
}
