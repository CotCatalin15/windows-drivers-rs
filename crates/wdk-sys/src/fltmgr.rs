// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(warnings)]

// allow wildcards for types module since underlying c code relies on all
// type definitions being in scope
#[allow(clippy::wildcard_imports)]
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/minifilter.rs"));

const IRP_MJ_OPERATION_END: u8 = 0x80;
