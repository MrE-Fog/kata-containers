# Copyright (c) 2019 Intel Corporation
# Copyright (c) 2020 Ant Group
#
# SPDX-License-Identifier: Apache-2.0
from ubuntu:20.04

# CACHE_TIMEOUT: date to invalid cache, if the date changes the image will be rebuild
# This is required to keep build dependencies with security fixes.
ARG CACHE_TIMEOUT
ARG DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && apt-get upgrade -y && \
    apt-get --no-install-recommends install -y \
	    apt-utils \
	    autoconf \
	    automake \
	    bc \
	    bison \
	    ca-certificates \
	    cpio \
	    flex \
	    gawk \
	    libaudit-dev \
	    libblkid-dev \
	    libcap-dev \
	    libcap-ng-dev \
	    libdw-dev \
	    libelf-dev \
	    libffi-dev \
	    libglib2.0-0 \
	    libglib2.0-dev \
	    libglib2.0-dev git \
	    libltdl-dev \
	    libmount-dev \
	    libpixman-1-dev \
	    libselinux1-dev \
	    libtool \
	    make \
	    ninja-build \
	    pkg-config \
	    libseccomp-dev \
	    libseccomp2 \
	    patch \
	    python \
	    python-dev \
	    rsync \
	    zlib1g-dev && \
    if [ "$(uname -m)" != "s390x" ]; then apt-get install -y --no-install-recommends libpmem-dev; fi && \
    apt-get clean && rm -rf /var/lib/apt/lists/

RUN git clone  https://github.com/axboe/liburing/ ~/liburing && \
    cd ~/liburing && \
    git checkout tags/liburing-2.1 && \
    make && make install && ldconfig
