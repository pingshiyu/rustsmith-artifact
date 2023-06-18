# rustsmith-artifact
Artifact for RustSmith tools paper.

# Obtaining the container
1. `git submodule update --init --progress`
2. `docker build -t rustsmith-artifact .`

# Running the container
`docker run -it -p 8080:8080 -p 8888:8888 --rm rustsmith-artifact`

# Container contents

## RustSmith binary
`rustsmith` contains the full RustSmith source code.

`rustsmith/bin/rustsmith` is the RustSmith binary, as used within the experiments of the paper.

## Code coverage
`code-coverage` contains the full results for code coverage of RustSmith / OOTS as quoted within the paper. 

The code coverage experiment can also be replicated using the below instructions.

### Viewing the full code coverage results as quoted in the paper
- `python -m http.server 8080 --bind 0.0.0.0 --directory /app/code-coverage/coverage`
- The coverage data can then be viewed from the host machine (outside the Docker container) at `localhost:8080`, within the respective `_html` folders.

### Re-running the code coverage experiments
- `cd code-coverage` 
- Clear existing coverage data:
    - `rm -r coverage/rustsmith/_html/* && rm -r coverage/oots/_html/*`
- For OOTS:
    - Generate `.profraw` files, for all tests in `mir-opt`, containing coverage information
        - `LLVM_PROFILE_FILE="coverage/oots/%p-%m.profraw" ./x.py test src/test/mir-opt --force-rerun`
    - Generate coverage `html` files
        - `grcov coverage/oots/*.profraw -s compiler -b /app/code-coverage/build/x86_64-unknown-linux-gnu --llvm-path /app/code-coverage/build/x86_64-unknown-linux-gnu/llvm/bin -t html -o coverage/oots/_html`
- For RustSmith:
    - Generate 1000 programs:
        - `/app/rustsmith/bin/rustsmith -n 1000 --directory coverage/rustsmith/files`
    - Compile generated programs to generate `.profraw` files:
        - `LLVM_PROFILE_FILE="coverage/rustsmith/%p-%m.profraw" python compile_rustsmith_files.py`
    - Generate coverage `html` files
        - `grcov coverage/rustsmith/*.profraw -s compiler -b /app/code-coverage/build/x86_64-unknown-linux-gnu --llvm-path /app/code-coverage/build/x86_64-unknown-linux-gnu/llvm/bin -t html -o coverage/rustsmith/_html`
- Viewing the results
    - Start up a simple Python server for the HTML files on port 8080:
        - `python -m http.server 8080 --bind 0.0.0.0 --directory /app/code-coverage/coverage`
    - The coverage data can then be viewed from the host machine (outside the Docker container) at `localhost:8080`, within the respective `_html` folders.

## Mutation coverage
`mutation-coverage` contains scripts to replicate the mutation coverage experiment (using the mutated `rustc` contained within `mutated-rustc` folder), for both the _official optimisations test suite_ (OOTS), and for RustSmith.

### First, ensure you are within `mutation-coverage` folder:
```bash
cd mutation-coverage
```

### Computing the mutation coverage of OOTS
```bash
python -m oots.coverage_by_harness
```
This should take around ~20 minutes to evaluate OOTS for all mutants.

### Computing mutation coverage of RustSmith, by executing the experiment described within the paper (identical settings), run:
```bash
python -m rustsmith.coverage_by_rustsmith --minutes-per-mutant 3
```
(Use `--help` option for more instructions on using this script and customise experiments). Running the above experiment will take ~18 hours.

### Compiling the results from the two mutation coverage experiments, and obtain processed results for the paper, run:
```bash
jupyter lab --notebook-dir=/app/mutation-coverage/analysis --ip=0.0.0.0 --port=8888 --allow-root
```
The notebook can be viewed at `localhost:8888` of the host machine, and to compile together the results, execute the notebook `compile_results.ipynb`.

Alternatively, execute the script for outputs to the command line:
```bash
python -m analysis.compile_results
```

## Historical bugs
`historical-bugs` folder contains the three bugs quoted within the paper.

These bugs can be replicated using the accompanying `triggers_bug.sh` script within each bug's folder, which compiles the files using the bug-triggering compiler settings, and verifies the output do indeed exhibit differences (i.e. bug detected via differential testing)

For example, replicating the v1.61 bug mentioned in the paper:
```bash
cd /app/historical-bugs/v1.61-invalid-opcode
./triggers_bug.sh
```

## Mutation coverage experiment results for RustSmith
`killed-mutants-controlled` contains the coverage results quoted for the RustSmith mutant-killing controlled experiment, as described in the paper.

Folders are named after the mutant IDs. If a RustSmith generated file kills the mutant, then the file is saved within a folder named `<KILL_REASON>_<hash>`. Each folder also contains an `info.json` file storing the metadata for the mutant's 3-minute killing round - the type of kill, and the time taken to find the kill within the experiment.