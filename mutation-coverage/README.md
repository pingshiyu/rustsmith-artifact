# Mutation Coverage Experiment Scripts

Scripts to replicate the mutation coverage experiment (using the mutated `rustc` contained within `mutated-rustc` folder), for both the _official optimisations test suite_ (OOTS), and for RustSmith.

To compute the mutation coverage of OOTS, run:
```bash
python -m oots.coverage_by_harness
```

To compute the mutation coverage of RustSmith, by executing the experiment described within the paper, run:
```bash
python -m rustsmith.coverage_by_rustsmith
```
(Use `--help` option for more instructions on using this script and customise experiments)

To compile the results from the two mutation coverage experiments, and obtain processed results for the paper, run:
```bash
bash
conda activate rustsmith-artifact
jupyter lab --notebook-dir=analysis --ip=0.0.0.0 --port=8888
```
and execute the notebook `read_results.ipynb`

Alternatively execute the script for outputs to the command line:
```bash
bash
conda activate rustsmith-artifact
python -m analysis.compile_results
```

Extra script to evaluate the mutation coverage of an existing set of test cases:
```bash
python -m rustsmith.kill_and_reduce --help
```
(Use `--help` for further instructions on its usage)