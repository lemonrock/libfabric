// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops
{
	pub size: usize,
	pub close: Option<unsafe extern "C" fn(fid: *mut fid) -> c_int>,
	pub bind: Option<unsafe extern "C" fn(fid: *mut fid, bfid: *mut fid, flags: u64) -> c_int>,
	pub control: Option<unsafe extern "C" fn(fid: *mut fid, command: c_int, arg: *mut c_void) -> c_int>,
	pub ops_open: Option<unsafe extern "C" fn(fid: *mut fid, name: *const c_char, flags: u64, ops: *mut *mut c_void, context: *mut c_void) -> c_int>,
}

impl Clone for fi_ops
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
