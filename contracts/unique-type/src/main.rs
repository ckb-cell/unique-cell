#![no_std]
#![cfg_attr(not(test), no_main)]

mod entry;
mod error;

#[cfg(test)]
extern crate alloc;

pub fn program_entry() -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
