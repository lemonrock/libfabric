// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_mr
{
	pub size: usize,
	pub reg: Option<unsafe extern "C" fn(fid: *mut fid, buf: *const c_void, len: usize, access: u64, offset: u64, requested_key: u64, flags: u64, mr: *mut *mut fid_mr, context: *mut c_void) -> c_int>,
	pub regv: Option<unsafe extern "C" fn(fid: *mut fid, iov: *const iovec, count: usize, access: u64, offset: u64, requested_key: u64, flags: u64, mr: *mut *mut fid_mr, context: *mut c_void) -> c_int>,
	pub regattr: Option<unsafe extern "C" fn(fid: *mut fid, attr: *const fi_mr_attr, flags: u64, mr: *mut *mut fid_mr) -> c_int>,
}

impl Clone for fi_ops_mr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_mr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
