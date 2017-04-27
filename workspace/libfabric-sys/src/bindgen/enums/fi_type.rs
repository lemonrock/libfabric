// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_type
{
	FI_TYPE_INFO = 0,
	FI_TYPE_EP_TYPE = 1,
	FI_TYPE_CAPS = 2,
	FI_TYPE_OP_FLAGS = 3,
	FI_TYPE_ADDR_FORMAT = 4,
	FI_TYPE_TX_ATTR = 5,
	FI_TYPE_RX_ATTR = 6,
	FI_TYPE_EP_ATTR = 7,
	FI_TYPE_DOMAIN_ATTR = 8,
	FI_TYPE_FABRIC_ATTR = 9,
	FI_TYPE_THREADING = 10,
	FI_TYPE_PROGRESS = 11,
	FI_TYPE_PROTOCOL = 12,
	FI_TYPE_MSG_ORDER = 13,
	FI_TYPE_MODE = 14,
	FI_TYPE_AV_TYPE = 15,
	FI_TYPE_ATOMIC_TYPE = 16,
	FI_TYPE_ATOMIC_OP = 17,
	FI_TYPE_VERSION = 18,
	FI_TYPE_EQ_EVENT = 19,
	FI_TYPE_CQ_EVENT_FLAGS = 20,
	FI_TYPE_MR_MODE = 21,
}
