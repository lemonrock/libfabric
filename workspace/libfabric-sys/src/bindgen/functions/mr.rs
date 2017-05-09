// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_mr_bind(mr: *mut fid_mr, bfid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_mr_desc(mr: *mut fid_mr) -> *mut c_void;
	pub fn rust_fi_mr_enable(mr: *mut fid_mr) -> c_int;
	pub fn rust_fi_mr_key(mr: *mut fid_mr) -> u64;
	pub fn rust_fi_mr_raw_attr(mr: *mut fid_mr, base_addr: *mut u64, raw_key: *mut u8, key_size: *mut usize, flags: u64) -> c_int;
	pub fn rust_fi_mr_refresh(mr: *mut fid_mr, iov: *const iovec, count: usize, flags: u64) -> c_int;
}
