#!/usr/bin/env sh

OBJECTDIR=/tmp/rustmodule
CURDIR=`pwd`

if [ ! -d "${OBJECTDIR}" ]; then
	mkdir "${OBJECTDIR}"
fi


make clean && \
	xargo build --target x86_64-kernel-freebsd && \
	cd "${OBJECTDIR}" && \
	ar -xv "${CURDIR}/target/x86_64-kernel-freebsd/debug/libmodule.a" && \
	cd "${CURDIR}" && \
	make OBJECTDIR="${OBJECTDIR}"
