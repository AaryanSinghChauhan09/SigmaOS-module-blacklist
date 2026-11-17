// Integration test for SigmaOS-module-blacklist
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use module_blacklist as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
