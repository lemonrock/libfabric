// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_cq_format
{
	FI_CQ_FORMAT_UNSPEC = 0,
	FI_CQ_FORMAT_CONTEXT = 1,
	FI_CQ_FORMAT_MSG = 2,
	FI_CQ_FORMAT_DATA = 3,
	FI_CQ_FORMAT_TAGGED = 4,
}
