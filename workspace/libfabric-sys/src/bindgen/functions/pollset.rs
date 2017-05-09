// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_poll(pollset: *mut fid_poll, context: *mut *mut c_void, count: c_int) -> c_int;
	pub fn rust_fi_poll_add(pollset: *mut fid_poll, event_fid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_poll_del(pollset: *mut fid_poll, event_fid: *mut fid, flags: u64) -> c_int;
}
