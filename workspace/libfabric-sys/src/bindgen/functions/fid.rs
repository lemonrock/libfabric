// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_alias(fid: *mut fid, alias_fid: *mut *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_cancel(fid: fid_t, context: *mut c_void) -> isize;
	pub fn rust_fi_close(fid: *mut fid) -> c_int;
	pub fn rust_fi_control(fid: *mut fid, command: c_int, arg: *mut c_void) -> c_int;
	pub fn rust_fi_getname(fid: fid_t, addr: *mut c_void, addrlen: *mut usize) -> c_int;
	pub fn rust_fi_getopt(fid: fid_t, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut usize) -> c_int;
	pub fn rust_fi_open_ops(fid: *mut fid, name: *const c_char, flags: u64, ops: *mut *mut c_void, context: *mut c_void) -> c_int;
	pub fn rust_fi_setname(fid: fid_t, addr: *mut c_void, addrlen: usize) -> c_int;
	pub fn rust_fi_setopt(fid: fid_t, level: c_int, optname: c_int, optval: *const c_void, optlen: usize) -> c_int;
}
