// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_poll
{
	pub size: usize,
	pub poll: Option<unsafe extern "C" fn(pollset: *mut fid_poll, context: *mut *mut c_void, count: c_int) -> c_int>,
	pub poll_add: Option<unsafe extern "C" fn(pollset: *mut fid_poll, event_fid: *mut fid, flags: u64) -> c_int>,
	pub poll_del: Option<unsafe extern "C" fn(pollset: *mut fid_poll, event_fid: *mut fid, flags: u64) -> c_int>,
}

impl Clone for fi_ops_poll
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_poll
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
