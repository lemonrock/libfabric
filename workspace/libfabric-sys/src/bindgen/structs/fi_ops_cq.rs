// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_cq
{
	pub size: usize,
	pub read: Option<unsafe extern "C" fn(cq: *mut fid_cq, buf: *mut c_void, count: usize) -> isize>,
	pub readfrom: Option<unsafe extern "C" fn(cq: *mut fid_cq, buf: *mut c_void, count: usize, src_addr: *mut fi_addr_t) -> isize>,
	pub readerr: Option<unsafe extern "C" fn(cq: *mut fid_cq, buf: *mut fi_cq_err_entry, flags: u64) -> isize>,
	pub sread: Option<unsafe extern "C" fn(cq: *mut fid_cq, buf: *mut c_void, count: usize, cond: *const c_void, timeout: c_int) -> isize>,
	pub sreadfrom: Option<unsafe extern "C" fn(cq: *mut fid_cq, buf: *mut c_void, count: usize, src_addr: *mut fi_addr_t, cond: *const c_void, timeout: c_int) -> isize>,
	pub signal: Option<unsafe extern "C" fn(cq: *mut fid_cq) -> c_int>,
	pub strerror: Option<unsafe extern "C" fn(cq: *mut fid_cq, prov_errno: c_int, err_data: *const c_void, buf: *mut c_char, len: usize) -> *const c_char>,
}

impl Clone for fi_ops_cq
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_cq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
