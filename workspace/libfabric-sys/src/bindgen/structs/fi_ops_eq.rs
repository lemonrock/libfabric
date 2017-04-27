// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_eq
{
	pub size: usize,
	pub read: Option<unsafe extern "C" fn(eq: *mut fid_eq, event: *mut u32, buf: *mut c_void, len: usize, flags: u64) -> isize>,
	pub readerr: Option<unsafe extern "C" fn(eq: *mut fid_eq, buf: *mut fi_eq_err_entry, flags: u64) -> isize>,
	pub write: Option<unsafe extern "C" fn(eq: *mut fid_eq, event: u32, buf: *const c_void, len: usize, flags: u64) -> isize>,
	pub sread: Option<unsafe extern "C" fn(eq: *mut fid_eq, event: *mut u32, buf: *mut c_void, len: usize, timeout: c_int, flags: u64) -> isize>,
	pub strerror: Option<unsafe extern "C" fn(eq: *mut fid_eq, prov_errno: c_int, err_data: *const c_void, buf: *mut c_char, len: usize) -> *const c_char>,
}

impl Clone for fi_ops_eq
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_eq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
