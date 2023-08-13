/*
 #![warn(
    absolute_paths_not_starting_with_crate,
    box_pointers,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences
)]
*/
//#![deny(warnings)]

// Modules
mod cli;
mod git;
mod utility;

use clap::Parser;

// Crates
use crate::cli::commands::*;
use crate::utility::settings::*;

fn main() {
    //let mut settings: Settings;
    initialize_settings();

    let args = MainArgs::parse();

    match_command(&args);

    if let Err(..) = write_settings(&get_settings()) {
        println!("An error occurred when writing settings.");
    };
}
