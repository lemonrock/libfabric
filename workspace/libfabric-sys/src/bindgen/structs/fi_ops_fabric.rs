// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_fabric
{
	pub size: usize,
	pub domain: Option<unsafe extern "C" fn(fabric: *mut fid_fabric, info: *mut fi_info, dom: *mut *mut fid_domain, context: *mut c_void) -> c_int>,
	pub passive_ep: Option<unsafe extern "C" fn(fabric: *mut fid_fabric, info: *mut fi_info, pep: *mut *mut fid_pep, context: *mut c_void) -> c_int>,
	pub eq_open: Option<unsafe extern "C" fn(fabric: *mut fid_fabric, attr: *mut fi_eq_attr, eq: *mut *mut fid_eq, context: *mut c_void) -> c_int>,
	pub wait_open: Option<unsafe extern "C" fn(fabric: *mut fid_fabric, attr: *mut fi_wait_attr, waitset: *mut *mut fid_wait) -> c_int>,
	pub trywait: Option<unsafe extern "C" fn(fabric: *mut fid_fabric, fids: *mut *mut fid, count: c_int) -> c_int>,
}

impl Clone for fi_ops_fabric
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_fabric
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
