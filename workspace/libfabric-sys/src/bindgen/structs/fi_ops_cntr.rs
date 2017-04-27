// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_cntr
{
	pub size: usize,
	pub read: Option<unsafe extern "C" fn(cntr: *mut fid_cntr) -> u64>,
	pub readerr: Option<unsafe extern "C" fn(cntr: *mut fid_cntr) -> u64>,
	pub add: Option<unsafe extern "C" fn(cntr: *mut fid_cntr, value: u64) -> c_int>,
	pub set: Option<unsafe extern "C" fn(cntr: *mut fid_cntr, value: u64) -> c_int>,
	pub wait: Option<unsafe extern "C" fn(cntr: *mut fid_cntr, threshold: u64, timeout: c_int) -> c_int>,
	pub adderr: Option<unsafe extern "C" fn(cntr: *mut fid_cntr, value: u64) -> c_int>,
	pub seterr: Option<unsafe extern "C" fn(cntr: *mut fid_cntr, value: u64) -> c_int>,
}

impl Clone for fi_ops_cntr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_cntr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
