// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_op
{
	FI_MIN = 0,
	FI_MAX = 1,
	FI_SUM = 2,
	FI_PROD = 3,
	FI_LOR = 4,
	FI_LAND = 5,
	FI_BOR = 6,
	FI_BAND = 7,
	FI_LXOR = 8,
	FI_BXOR = 9,
	FI_ATOMIC_READ = 10,
	FI_ATOMIC_WRITE = 11,
	FI_CSWAP = 12,
	FI_CSWAP_NE = 13,
	FI_CSWAP_LE = 14,
	FI_CSWAP_LT = 15,
	FI_CSWAP_GE = 16,
	FI_CSWAP_GT = 17,
	FI_MSWAP = 18,
	FI_ATOMIC_OP_LAST = 19,
}
