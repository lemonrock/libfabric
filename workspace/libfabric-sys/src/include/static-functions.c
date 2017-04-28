// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#define FABRIC_DIRECT
#include <rdma/fabric.h>
#include <rdma/fi_atomic.h>
#include <rdma/fi_cm.h>
#include <rdma/fi_domain.h>
#include <rdma/fi_endpoint.h>
#include <rdma/fi_eq.h>
#include <rdma/fi_errno.h>
#include <rdma/fi_rma.h>
#include <rdma/fi_tagged.h>
#include <rdma/fi_trigger.h>

struct fi_info * rust_fi_allocinfo()
{
	return fi_allocinfo();
}

int rust_fi_close(struct fid * fid)
{
	return fi_close(fid);
}

int rust_fi_control(struct fid * fid, int command, void * arg)
{
	return fi_control(fid, command, arg);
}

int rust_fi_alias(struct fid * fid, struct fid * * alias_fid, uint64_t flags)
{
	return fi_alias(fid, alias_fid, flags);
}

int rust_fi_open_ops(struct fid * fid, const char * name, uint64_t flags, void * * ops, void * context)
{
	return fi_open_ops(fid, name, flags, ops, context);
}

ssize_t rust_fi_atomic(struct fid_ep * ep, const void * buf, size_t count, void * desc, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)
{
	return fi_atomic(ep, buf, count, desc, dest_addr, addr, key, datatype, op, context);
}

ssize_t rust_fi_atomicv(struct fid_ep * ep, const struct fi_ioc * iov, void * * desc, size_t count, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)
{
	return fi_atomicv(ep, iov, desc, count, dest_addr, addr, key, datatype, op, context);
}

ssize_t rust_fi_atomicmsg(struct fid_ep * ep, const struct fi_msg_atomic * msg, uint64_t flags)
{
	return fi_atomicmsg(ep, msg, flags);
}

ssize_t rust_fi_inject_atomic(struct fid_ep * ep, const void * buf, size_t count, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op)
{
	return fi_inject_atomic(ep, buf, count, dest_addr, addr, key, datatype, op);
}

ssize_t rust_fi_fetch_atomic(struct fid_ep * ep, const void * buf, size_t count, void * desc, void * result, void * result_desc, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)
{
	return fi_fetch_atomic(ep, buf, count, desc, result, result_desc, dest_addr, addr, key, datatype, op, context);
}

// ssize_t fi_fetch_atomicv(struct fid_ep * ep, const struct fi_ioc * iov, void * * desc, size_t count, struct fi_ioc * resultv, void * * result_desc, size_t result_count, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)

// ssize_t fi_fetch_atomicmsg(struct fid_ep * ep, const struct fi_msg_atomic * msg, struct fi_ioc * resultv, void * * result_desc, size_t result_count, uint64_t flags)

// ssize_t fi_compare_atomic(struct fid_ep * ep, const void * buf, size_t count, void * desc, const void * compare, void * compare_desc, void * result, void * result_desc, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)

// ssize_t fi_compare_atomicv(struct fid_ep * ep, const struct fi_ioc * iov, void * * desc, size_t count, const struct fi_ioc * comparev, void * * compare_desc, size_t compare_count, struct fi_ioc * resultv, void * * result_desc, size_t result_count, fi_addr_t dest_addr, uint64_t addr, uint64_t key, enum fi_datatype datatype, enum fi_op op, void * context)

// ssize_t fi_compare_atomicmsg(struct fid_ep * ep, const struct fi_msg_atomic * msg, const struct fi_ioc * comparev, void * * compare_desc, size_t compare_count, struct fi_ioc * resultv, void * * result_desc, size_t result_count, uint64_t flags)

// int fi_atomicvalid(struct fid_ep * ep, enum fi_datatype datatype, enum fi_op op, size_t * count)

// int fi_fetch_atomicvalid(struct fid_ep * ep, enum fi_datatype datatype, enum fi_op op, size_t * count)

// int fi_compare_atomicvalid(struct fid_ep * ep, enum fi_datatype datatype, enum fi_op op, size_t * count)

// int fi_query_atomic(struct fid_domain * domain, enum fi_datatype datatype, enum fi_op op, struct fi_atomic_attr * attr, uint64_t flags)

// int fi_setname(fid_t fid, void * addr, size_t addrlen)

// int fi_getname(fid_t fid, void * addr, size_t * addrlen)

// int fi_getpeer(struct fid_ep * ep, void * addr, size_t * addrlen)

// int fi_listen(struct fid_pep * pep)

// int fi_connect(struct fid_ep * ep, const void * addr, const void * param, size_t paramlen)

// int fi_accept(struct fid_ep * ep, const void * param, size_t paramlen)

// int fi_reject(struct fid_pep * pep, fid_t handle, const void * param, size_t paramlen)

// int fi_shutdown(struct fid_ep * ep, uint64_t flags)

// int fi_join(struct fid_ep * ep, const void * addr, uint64_t flags, struct fid_mc * * mc, void * context)

// fi_addr_t fi_mc_addr(struct fid_mc * mc)

// int fi_domain(struct fid_fabric * fabric, struct fi_info * info, struct fid_domain * * domain, void * context)

// int fi_domain_bind(struct fid_domain * domain, struct fid * fid, uint64_t flags)

// int fi_cq_open(struct fid_domain * domain, struct fi_cq_attr * attr, struct fid_cq * * cq, void * context)

// int fi_cntr_open(struct fid_domain * domain, struct fi_cntr_attr * attr, struct fid_cntr * * cntr, void * context)

// int fi_wait_open(struct fid_fabric * fabric, struct fi_wait_attr * attr, struct fid_wait * * waitset)

// int fi_poll_open(struct fid_domain * domain, struct fi_poll_attr * attr, struct fid_poll * * pollset)

// int fi_mr_reg(struct fid_domain * domain, const void * buf, size_t len, uint64_t access, uint64_t offset, uint64_t requested_key, uint64_t flags, struct fid_mr * * mr, void * context)

// int fi_mr_regv(struct fid_domain * domain, const struct iovec * iov, size_t count, uint64_t access, uint64_t offset, uint64_t requested_key, uint64_t flags, struct fid_mr * * mr, void * context)

// int fi_mr_regattr(struct fid_domain * domain, const struct fi_mr_attr * attr, uint64_t flags, struct fid_mr * * mr)

// void * fi_mr_desc(struct fid_mr * mr)

// uint64_t fi_mr_key(struct fid_mr * mr)

// int fi_mr_raw_attr(struct fid_mr * mr, uint64_t * base_addr, uint8_t * raw_key, size_t * key_size, uint64_t flags)

// int fi_mr_map_raw(struct fid_domain * domain, uint64_t base_addr, uint8_t * raw_key, size_t key_size, uint64_t * key, uint64_t flags)

// int fi_mr_unmap_key(struct fid_domain * domain, uint64_t key)

// int fi_mr_bind(struct fid_mr * mr, struct fid * bfid, uint64_t flags)

// int fi_mr_refresh(struct fid_mr * mr, const struct iovec * iov, size_t count, uint64_t flags)

// int fi_mr_enable(struct fid_mr * mr)

// int fi_av_open(struct fid_domain * domain, struct fi_av_attr * attr, struct fid_av * * av, void * context)

// int fi_av_bind(struct fid_av * av, struct fid * fid, uint64_t flags)

// int fi_av_insert(struct fid_av * av, const void * addr, size_t count, fi_addr_t * fi_addr, uint64_t flags, void * context)

// int fi_av_insertsvc(struct fid_av * av, const char * node, const char * service, fi_addr_t * fi_addr, uint64_t flags, void * context)

// int fi_av_insertsym(struct fid_av * av, const char * node, size_t nodecnt, const char * service, size_t svccnt, fi_addr_t * fi_addr, uint64_t flags, void * context)

// int fi_av_remove(struct fid_av * av, fi_addr_t * fi_addr, size_t count, uint64_t flags)

// int fi_av_lookup(struct fid_av * av, fi_addr_t fi_addr, void * addr, size_t * addrlen)

// const char * fi_av_straddr(struct fid_av * av, const void * addr, char * buf, size_t * len)

// fi_addr_t fi_rx_addr(fi_addr_t fi_addr, int rx_index, int rx_ctx_bits)

// int fi_passive_ep(struct fid_fabric * fabric, struct fi_info * info, struct fid_pep * * pep, void * context)

// int fi_endpoint(struct fid_domain * domain, struct fi_info * info, struct fid_ep * * ep, void * context)

// int fi_scalable_ep(struct fid_domain * domain, struct fi_info * info, struct fid_ep * * sep, void * context)

// int fi_ep_bind(struct fid_ep * ep, struct fid * bfid, uint64_t flags)

// int fi_pep_bind(struct fid_pep * pep, struct fid * bfid, uint64_t flags)

// int fi_scalable_ep_bind(struct fid_ep * sep, struct fid * bfid, uint64_t flags)

// int fi_enable(struct fid_ep * ep)

// ssize_t fi_cancel(fid_t fid, void * context)

// int fi_setopt(fid_t fid, int level, int optname, const void * optval, size_t optlen)

// int fi_getopt(fid_t fid, int level, int optname, void * optval, size_t * optlen)

// int fi_ep_alias(struct fid_ep * ep, struct fid_ep * * alias_ep, uint64_t flags)

// int fi_tx_context(struct fid_ep * ep, int index, struct fi_tx_attr * attr, struct fid_ep * * tx_ep, void * context)

// int fi_rx_context(struct fid_ep * ep, int index, struct fi_rx_attr * attr, struct fid_ep * * rx_ep, void * context)

// FI_DEPRECATED_FUNC ssize_t fi_rx_size_left(struct fid_ep * ep)

// FI_DEPRECATED_FUNC ssize_t fi_tx_size_left(struct fid_ep * ep)

// int fi_stx_context(struct fid_domain * domain, struct fi_tx_attr * attr, struct fid_stx * * stx, void * context)

// int fi_srx_context(struct fid_domain * domain, struct fi_rx_attr * attr, struct fid_ep * * rx_ep, void * context)

// ssize_t fi_recv(struct fid_ep * ep, void * buf, size_t len, void * desc, fi_addr_t src_addr, void * context)

// ssize_t fi_recvv(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t src_addr, void * context)

// ssize_t fi_recvmsg(struct fid_ep * ep, const struct fi_msg * msg, uint64_t flags)

// ssize_t fi_send(struct fid_ep * ep, const void * buf, size_t len, void * desc, fi_addr_t dest_addr, void * context)

// ssize_t fi_sendv(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t dest_addr, void * context)

// ssize_t fi_sendmsg(struct fid_ep * ep, const struct fi_msg * msg, uint64_t flags)

// ssize_t fi_inject(struct fid_ep * ep, const void * buf, size_t len, fi_addr_t dest_addr)

// ssize_t fi_senddata(struct fid_ep * ep, const void * buf, size_t len, void * desc, uint64_t data, fi_addr_t dest_addr, void * context)

// ssize_t fi_injectdata(struct fid_ep * ep, const void * buf, size_t len, uint64_t data, fi_addr_t dest_addr)

// int fi_trywait(struct fid_fabric * fabric, struct fid * * fids, int count)

// int fi_wait(struct fid_wait * waitset, int timeout)

// int fi_poll(struct fid_poll * pollset, void * * context, int count)

// int fi_poll_add(struct fid_poll * pollset, struct fid * event_fid, uint64_t flags)

// int fi_poll_del(struct fid_poll * pollset, struct fid * event_fid, uint64_t flags)

// int fi_eq_open(struct fid_fabric * fabric, struct fi_eq_attr * attr, struct fid_eq * * eq, void * context)

// ssize_t fi_eq_read(struct fid_eq * eq, uint32_t * event, void * buf, size_t len, uint64_t flags)

// ssize_t fi_eq_readerr(struct fid_eq * eq, struct fi_eq_err_entry * buf, uint64_t flags)

// ssize_t fi_eq_write(struct fid_eq * eq, uint32_t event, const void * buf, size_t len, uint64_t flags)

// ssize_t fi_eq_sread(struct fid_eq * eq, uint32_t * event, void * buf, size_t len, int timeout, uint64_t flags)

// const char * fi_eq_strerror(struct fid_eq * eq, int prov_errno, const void * err_data, char * buf, size_t len)

// ssize_t fi_cq_read(struct fid_cq * cq, void * buf, size_t count)

// ssize_t fi_cq_readfrom(struct fid_cq * cq, void * buf, size_t count, fi_addr_t * src_addr)

// ssize_t fi_cq_readerr(struct fid_cq * cq, struct fi_cq_err_entry * buf, uint64_t flags)

// ssize_t fi_cq_sread(struct fid_cq * cq, void * buf, size_t count, const void * cond, int timeout)

// ssize_t fi_cq_sreadfrom(struct fid_cq * cq, void * buf, size_t count, fi_addr_t * src_addr, const void * cond, int timeout)

// int fi_cq_signal(struct fid_cq * cq)

// const char * fi_cq_strerror(struct fid_cq * cq, int prov_errno, const void * err_data, char * buf, size_t len)

// uint64_t fi_cntr_read(struct fid_cntr * cntr)

// uint64_t fi_cntr_readerr(struct fid_cntr * cntr)

// int fi_cntr_add(struct fid_cntr * cntr, uint64_t value)

// int fi_cntr_adderr(struct fid_cntr * cntr, uint64_t value)

// int fi_cntr_set(struct fid_cntr * cntr, uint64_t value)

// int fi_cntr_seterr(struct fid_cntr * cntr, uint64_t value)

// int fi_cntr_wait(struct fid_cntr * cntr, uint64_t threshold, int timeout)

// ssize_t fi_read(struct fid_ep * ep, void * buf, size_t len, void * desc, fi_addr_t src_addr, uint64_t addr, uint64_t key, void * context)

// ssize_t fi_readv(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t src_addr, uint64_t addr, uint64_t key, void * context)

// ssize_t fi_readmsg(struct fid_ep * ep, const struct fi_msg_rma * msg, uint64_t flags)

// ssize_t fi_write(struct fid_ep * ep, const void * buf, size_t len, void * desc, fi_addr_t dest_addr, uint64_t addr, uint64_t key, void * context)

// ssize_t fi_writev(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t dest_addr, uint64_t addr, uint64_t key, void * context)

// ssize_t fi_writemsg(struct fid_ep * ep, const struct fi_msg_rma * msg, uint64_t flags)

// ssize_t fi_inject_write(struct fid_ep * ep, const void * buf, size_t len, fi_addr_t dest_addr, uint64_t addr, uint64_t key)

// ssize_t fi_writedata(struct fid_ep * ep, const void * buf, size_t len, void * desc, uint64_t data, fi_addr_t dest_addr, uint64_t addr, uint64_t key, void * context)

// ssize_t fi_inject_writedata(struct fid_ep * ep, const void * buf, size_t len, uint64_t data, fi_addr_t dest_addr, uint64_t addr, uint64_t key)

// ssize_t fi_trecv(struct fid_ep * ep, void * buf, size_t len, void * desc, fi_addr_t src_addr, uint64_t tag, uint64_t ignore, void * context)

// ssize_t fi_trecvv(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t src_addr, uint64_t tag, uint64_t ignore, void * context)

// ssize_t fi_trecvmsg(struct fid_ep * ep, const struct fi_msg_tagged * msg, uint64_t flags)

// ssize_t fi_tsend(struct fid_ep * ep, const void * buf, size_t len, void * desc, fi_addr_t dest_addr, uint64_t tag, void * context)

// ssize_t fi_tsendv(struct fid_ep * ep, const struct iovec * iov, void * * desc, size_t count, fi_addr_t dest_addr, uint64_t tag, void * context)

// ssize_t fi_tsendmsg(struct fid_ep * ep, const struct fi_msg_tagged * msg, uint64_t flags)

// ssize_t fi_tinject(struct fid_ep * ep, const void * buf, size_t len, fi_addr_t dest_addr, uint64_t tag)

// ssize_t fi_tsenddata(struct fid_ep * ep, const void * buf, size_t len, void * desc, uint64_t data, fi_addr_t dest_addr, uint64_t tag, void * context)

// ssize_t fi_tinjectdata(struct fid_ep * ep, const void * buf, size_t len, uint64_t data, fi_addr_t dest_addr, uint64_t tag)

