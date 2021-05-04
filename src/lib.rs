#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod luigi;
mod luigieffect;
mod luigiexpression;
mod luigisound;

#[skyline::main(name = "swole_luigi")]
pub fn main() {
    luigi::install();
    luigieffect::install();
    luigiexpression::install();
    luigisound::install()
;}