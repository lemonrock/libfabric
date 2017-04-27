// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_av
{
	pub size: usize,
	pub insert: Option<unsafe extern "C" fn(av: *mut fid_av, addr: *const c_void, count: usize, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int>,
	pub insertsvc: Option<unsafe extern "C" fn(av: *mut fid_av, node: *const c_char, service: *const c_char, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int>,
	pub insertsym: Option<unsafe extern "C" fn(av: *mut fid_av, node: *const c_char, nodecnt: usize, service: *const c_char, svccnt: usize, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(av: *mut fid_av, fi_addr: *mut fi_addr_t, count: usize, flags: u64) -> c_int>,
	pub lookup: Option<unsafe extern "C" fn(av: *mut fid_av, fi_addr: fi_addr_t, addr: *mut c_void, addrlen: *mut usize) -> c_int>,
	pub straddr: Option<unsafe extern "C" fn(av: *mut fid_av, addr: *const c_void, buf: *mut c_char, len: *mut usize) -> *const c_char>,
}

impl Clone for fi_ops_av
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_av
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
