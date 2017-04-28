// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn fi_strerror(errnum: c_int) -> *const c_char;
	pub fn fi_tostr(data: *const c_void, datatype: fi_type) -> *mut c_char;
	pub fn fi_version() -> u32;
	pub fn rust_fi_alias(fid: *mut fid, alias_fid: *mut *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_allocinfo() -> *mut fi_info;
	pub fn rust_fi_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_atomicmsg(ep: *mut fid_ep, msg: *const fi_msg_atomic, flags: u64) -> isize;
	pub fn rust_fi_atomicv(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_close(fid: *mut fid) -> c_int;
	pub fn rust_fi_control(fid: *mut fid, command: c_int, arg: *mut c_void) -> c_int;
	pub fn rust_fi_fetch_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, result: *mut c_void, result_desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_inject_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op) -> isize;
	pub fn rust_fi_open_ops(fid: *mut fid, name: *const c_char, flags: u64, ops: *mut *mut c_void, context: *mut c_void) -> c_int;
}
