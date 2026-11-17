// Integration test for SigmaOS-distributed-tracing
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use distributed_tracing as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
