//! Saby/c - Virtual Machine for mruby byte-code.
//!
//!  Copyright (C) 2021 ABplus Inc. kazhida
//!
//! This file is distributed under BSD 3-Clause License.
//!

mod config;
mod vm;
mod builtin;
mod op_code;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
