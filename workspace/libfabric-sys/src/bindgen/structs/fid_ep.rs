// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fid_ep
{
	pub fid: fid,
	pub ops: *mut fi_ops_ep,
	pub cm: *mut fi_ops_cm,
	pub msg: *mut fi_ops_msg,
	pub rma: *mut fi_ops_rma,
	pub tagged: *mut fi_ops_tagged,
	pub atomic: *mut fi_ops_atomic,
}

impl Clone for fid_ep
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fid_ep
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
