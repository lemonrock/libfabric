// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum fi_datatype
{
	FI_INT8 = 0,
	FI_UINT8 = 1,
	FI_INT16 = 2,
	FI_UINT16 = 3,
	FI_INT32 = 4,
	FI_UINT32 = 5,
	FI_INT64 = 6,
	FI_UINT64 = 7,
	FI_FLOAT = 8,
	FI_DOUBLE = 9,
	FI_FLOAT_COMPLEX = 10,
	FI_DOUBLE_COMPLEX = 11,
	FI_LONG_DOUBLE = 12,
	FI_LONG_DOUBLE_COMPLEX = 13,
	FI_DATATYPE_LAST = 14,
}
