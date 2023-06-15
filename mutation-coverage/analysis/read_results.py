import shelve
from pathlib import Path
from typing import Dict

import pandas as pd
from collections import Counter

from settings import Detection

def read_results(store_path: Path) -> Dict:
    """
    Reads a coverage Dict of the type:
        string -> [Any]
    """
    coverage = {}
    with shelve.open(str(store_path)) as results:
        errors = 0
        for i, k in enumerate(results.keys()):
            try:
                coverage[k] = results[k]
                # print(f"#{i+1}: {k} ->", results[k])
            except EOFError:
                errors += 1
        
        print("Errors:", errors)
    return coverage

def peek_dict(mapping: dict, first: int = 3) -> dict:
    # peek into a dict, to get an idea of what the data looks like
    return {
        key_value[0]: key_value[1] for 
        i, key_value in enumerate(mapping.items())
        if i < first
    }

def transpose_coverage_results(coverage: Dict) -> Dict:
    """
    Converts a coverage record of mapping:
        filename -> [(mutation, Detection)]
    Into a mapping:
        mutation -> [(filename, Detection)]
    """
    coverage_t = {}
    for file, kills in coverage.items():
        if not kills or kills[-1][0] == -1: # no kills or aborted due to too many timeouts
            continue
        for mutant, kill_method in kills:
            coverage_t[mutant] = coverage_t.get(mutant, []) + [(file, kill_method)]
    return coverage_t