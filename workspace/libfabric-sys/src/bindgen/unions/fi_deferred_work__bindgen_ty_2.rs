// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union fi_deferred_work__bindgen_ty_2
{
    pub msg: *mut fi_op_msg,
    pub tagged: *mut fi_op_tagged,
    pub rma: *mut fi_op_rma,
    pub atomic: *mut fi_op_atomic,
    pub fetch_atomic: *mut fi_op_fetch_atomic,
    pub compare_atomic: *mut fi_op_compare_atomic,
    pub cntr: *mut fi_op_cntr,
}

impl Clone for fi_deferred_work__bindgen_ty_2
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_deferred_work__bindgen_ty_2
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for fi_deferred_work
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_deferred_work
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
