// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct fi_deferred_work
{
	pub context: fi_context,
	pub event_type: fi_trigger_event,
	pub op_type: fi_trigger_op,
	pub event: fi_deferred_work__bindgen_ty_1,
	pub op: fi_deferred_work__bindgen_ty_2,
}
