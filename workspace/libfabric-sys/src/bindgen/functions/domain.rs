// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_av_open(domain: *mut fid_domain, attr: *mut fi_av_attr, av: *mut *mut fid_av, context: *mut c_void) -> c_int;
	pub fn rust_fi_cntr_open(domain: *mut fid_domain, attr: *mut fi_cntr_attr, cntr: *mut *mut fid_cntr, context: *mut c_void) -> c_int;
	pub fn rust_fi_cq_open(domain: *mut fid_domain, attr: *mut fi_cq_attr, cq: *mut *mut fid_cq, context: *mut c_void) -> c_int;
	pub fn rust_fi_domain_bind(domain: *mut fid_domain, fid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_endpoint(domain: *mut fid_domain, info: *mut fi_info, ep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	pub fn rust_fi_mr_map_raw(domain: *mut fid_domain, base_addr: u64, raw_key: *mut u8, key_size: usize, key: *mut u64, flags: u64) -> c_int;
	pub fn rust_fi_mr_reg(domain: *mut fid_domain, buf: *const c_void, len: usize, access: u64, offset: u64, requested_key: u64, flags: u64, mr: *mut *mut fid_mr, context: *mut c_void) -> c_int;
	pub fn rust_fi_mr_regattr(domain: *mut fid_domain, attr: *const fi_mr_attr, flags: u64, mr: *mut *mut fid_mr) -> c_int;
	pub fn rust_fi_mr_regv(domain: *mut fid_domain, iov: *const iovec, count: usize, access: u64, offset: u64, requested_key: u64, flags: u64, mr: *mut *mut fid_mr, context: *mut c_void) -> c_int;
	pub fn rust_fi_mr_unmap_key(domain: *mut fid_domain, key: u64) -> c_int;
	pub fn rust_fi_poll_open(domain: *mut fid_domain, attr: *mut fi_poll_attr, pollset: *mut *mut fid_poll) -> c_int;
	pub fn rust_fi_query_atomic(domain: *mut fid_domain, datatype: fi_datatype, op: fi_op, attr: *mut fi_atomic_attr, flags: u64) -> c_int;
	pub fn rust_fi_scalable_ep(domain: *mut fid_domain, info: *mut fi_info, sep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	pub fn rust_fi_srx_context(domain: *mut fid_domain, attr: *mut fi_rx_attr, rx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	pub fn rust_fi_stx_context(domain: *mut fid_domain, attr: *mut fi_tx_attr, stx: *mut *mut fid_stx, context: *mut c_void) -> c_int;
}
