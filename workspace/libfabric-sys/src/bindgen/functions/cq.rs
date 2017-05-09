// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_cq_read(cq: *mut fid_cq, buf: *mut c_void, count: usize) -> isize;
	pub fn rust_fi_cq_readerr(cq: *mut fid_cq, buf: *mut fi_cq_err_entry, flags: u64) -> isize;
	pub fn rust_fi_cq_readfrom(cq: *mut fid_cq, buf: *mut c_void, count: usize, src_addr: *mut fi_addr_t) -> isize;
	pub fn rust_fi_cq_signal(cq: *mut fid_cq) -> c_int;
	pub fn rust_fi_cq_sread(cq: *mut fid_cq, buf: *mut c_void, count: usize, cond: *const c_void, timeout: c_int) -> isize;
	pub fn rust_fi_cq_sreadfrom(cq: *mut fid_cq, buf: *mut c_void, count: usize, src_addr: *mut fi_addr_t, cond: *const c_void, timeout: c_int) -> isize;
	pub fn rust_fi_cq_strerror(cq: *mut fid_cq, prov_errno: c_int, err_data: *const c_void, buf: *mut c_char, len: usize) -> *const c_char;
}
