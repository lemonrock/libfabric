// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_cq_err_entry
{
	pub op_context: *mut c_void,
	pub flags: u64,
	pub len: usize,
	pub buf: *mut c_void,
	pub data: u64,
	pub tag: u64,
	pub olen: usize,
	pub err: c_int,
	pub prov_errno: c_int,
	pub err_data: *mut c_void,
	pub err_data_size: usize,
}

impl Clone for fi_cq_err_entry
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_cq_err_entry
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
