// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_trigger_op
{
	FI_OP_RECV = 0,
	FI_OP_SEND = 1,
	FI_OP_TRECV = 2,
	FI_OP_TSEND = 3,
	FI_OP_READ = 4,
	FI_OP_WRITE = 5,
	FI_OP_ATOMIC = 6,
	FI_OP_FETCH_ATOMIC = 7,
	FI_OP_COMPARE_ATOMIC = 8,
	FI_OP_CNTR_SET = 9,
	FI_OP_CNTR_ADD = 10,
}
