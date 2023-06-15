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

To compile the results from the two mutation coverage experiments, and obtain processed results for the paper, run:
```bash
jupyter lab --notebook-dir=analysis
```
and execute the notebook `read_results.ipynb`