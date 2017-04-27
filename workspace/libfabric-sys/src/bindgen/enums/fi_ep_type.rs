// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_ep_type
{
	FI_EP_UNSPEC = 0,
	FI_EP_MSG = 1,
	FI_EP_DGRAM = 2,
	FI_EP_RDM = 3,
	FI_EP_SOCK_STREAM = 4,
	FI_EP_SOCK_DGRAM = 5,
}
