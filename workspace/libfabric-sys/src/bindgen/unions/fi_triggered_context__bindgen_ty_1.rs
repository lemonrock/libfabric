// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union fi_triggered_context__bindgen_ty_1
{
    pub threshold: fi_trigger_threshold,
    pub internal: [*mut c_void; 3usize],
}

impl Clone for fi_triggered_context__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_triggered_context__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for fi_triggered_context
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_triggered_context
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
