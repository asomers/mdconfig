#! /bin/sh

CRATEDIR=`dirname $0`/..

cat > src/ffi.rs << HERE
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
HERE

bindgen --allowlist-type 'md_ioctl' \
	--allowlist-item 'MDIOVERSION' \
	--allowlist-item 'MDNPAD' \
	--allowlist-item 'MD_ASYNC' \
	--allowlist-item 'MD_AUTOUNIT' \
	--allowlist-item 'MD_CACHE' \
	--allowlist-item 'MD_CLUSTER' \
	--allowlist-item 'MD_COMPRESS' \
	--allowlist-item 'MD_VERIFY' \
	--allowlist-item 'MD_FORCE' \
	--allowlist-item 'MD_MUSTDEALLOC' \
	--allowlist-item 'MD_READONLY' \
	--allowlist-item 'MD_RESERVE' \
	--allowlist-item 'MD_VERIFY' \
	${CRATEDIR}/bindgen/wrapper.h >> ${CRATEDIR}/src/ffi.rs

cat > tests/ffi.rs << HERE
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
HERE
bindgen --allowlist-type diocgattr_arg /usr/include/sys/disk.h >> ${CRATEDIR}/tests/ffi.rs