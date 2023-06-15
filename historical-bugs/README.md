# Historical Bugs 

This folder contains the historical bugs described in the paper, found by using RustSmith as a program generator.

To replicate the historical bugs, run the `triggers_bug.sh` script within each folder. The script will execute the Rust script using pre-defined compiler settings, and compare the generated binaries (compare contents, and execution) to trigger the bug.