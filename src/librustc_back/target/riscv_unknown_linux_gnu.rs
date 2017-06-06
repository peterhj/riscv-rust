// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "riscv-unknown-linux-gnu".to_string(),
        target_endian: "little".to_string(),
        /*target_pointer_width: "64".to_string(),
        data_layout: "e-m:e-i64:64-n32-S128".to_string(), // XXX: the AB version.*/
        target_pointer_width: "32".to_string(),
        data_layout: "e-m:e-p:32:32:32-i1:8:16-i8:8:16-i16:16-i32:32-f32:32-f64:64-f80:128-f128:128-n32".to_string(),
        arch: "riscv".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            cpu: "RV32IMAFD".to_string(),
            features: "".to_string(),
            max_atomic_width: Some(32),

            // see #36994
            exe_allocation_crate: "alloc_system".to_string(),

            ..super::linux_base::opts()
        },
    })
}
