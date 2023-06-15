#!/usr/bin/env python
# coding: utf-8

# # Notebook content
# - Find mutants killed by RustSmith
# - Retrieve mutant killing data of official test suite
# - Find mutants killed more strongly by RustSmith (above Bin_Difference strength)
# - Find time taken to kill each mutant killed

# In[1]:
import sys
sys.path.append("../")

from analysis.read_results import read_results
from settings import Detection, DETECTION_CODE, ALL_MUTANTS, MUT_COVERAGE_EXPERIMENT_ROOT

from pathlib import Path
import pandas as pd
import json


# # OOTS mutant killing results

# In[2]:


results_harness = read_results(MUT_COVERAGE_EXPERIMENT_ROOT / "_results/oots/store")
results_harness_summaries = {int(m): data['summary'] for m, data in results_harness.items()}
results_harness_df = pd.DataFrame.from_dict(results_harness_summaries).T


# In[3]:


ots_killed = results_harness_df[results_harness_df.STATUS != 'ok'].index


# In[4]:


print(results_harness_df)


# # RustSmith mutant killing results

# In[5]:


CODE_DETECTION = {
    code: detection for detection, code in DETECTION_CODE.items()
}


# In[6]:


# scan through ../killing_ground/out/info.json
# unpack json files for mutant killing strength, and time
mutant_kill_method = {}

killing_ground_p = MUT_COVERAGE_EXPERIMENT_ROOT / "_results/rustsmith/"
for json_fp in killing_ground_p.glob("*/*.json"):
    record = None
    with open(json_fp) as f:
        record = json.load(f)
    
    if record is None:
        raise Exception("File wasn't able to be read")
    
    kill_methods = {}
    for code, kill_details in record.items():
        _, time_taken = kill_details # filename, time taken
        kill_methods[CODE_DETECTION[int(code)]] = time_taken
        
    mutant_id = int(json_fp.parent.name)
    mutant_kill_method[mutant_id] = kill_methods # Counter(kill_methods)


# In[7]:


rustsmith_rr_times_df = pd.DataFrame.from_dict(mutant_kill_method).fillna(-1).T.sort_index()
rustsmith_rr_df = rustsmith_rr_times_df > 0

# ignoring COMPILE_TIMEOUT detections
rustsmith_rr_strong_df = rustsmith_rr_df.drop(columns=[Detection.COMPILE_TIMEOUT])
print(rustsmith_rr_strong_df)


# In[8]:


print(rustsmith_rr_times_df.drop(columns=[Detection.COMPILE_TIMEOUT]))


# In[9]:


# all mutants with some kills
killed_mutants_idx = rustsmith_rr_strong_df.any(axis=1)
rustsmith_rr_strong_kills_df = rustsmith_rr_strong_df[killed_mutants_idx]


# In[10]:


rustsmith_kills = list(rustsmith_rr_strong_kills_df.index)


# # Overall killings comparison

# In[11]:


# table will have form
#                rustsmith killed | rustsmith unkilled
# OTS killed   |
# OTS unkilled |
ots_killed_set = set(ots_killed)
rustsmith_killed_set = set(rustsmith_kills)

neither_killed_set = set(ALL_MUTANTS) - ots_killed_set - rustsmith_killed_set
both_killed = ots_killed_set & rustsmith_killed_set
rustsmith_not_ots = rustsmith_killed_set - ots_killed_set
ots_not_rustsmith = ots_killed_set - rustsmith_killed_set

print(f"{len(both_killed)=}, {len(rustsmith_not_ots)=}, {len(ots_not_rustsmith)=}, {len(neither_killed_set)=}")


# # Kill Strengths Evaluation

# In[12]:


rustsmith_rr_adjusted_strong_kills_df = rustsmith_rr_strong_kills_df.copy()
stronger_kill_adjustment = rustsmith_rr_strong_kills_df[[Detection.OUTPUT_ERRORS, Detection.OUTPUT_DIFFERENCE, Detection.OUTPUT_TIMEOUT]].any(axis=1)
rustsmith_rr_adjusted_strong_kills_df[Detection.BINARY_DIFFERENCE] = (
    rustsmith_rr_adjusted_strong_kills_df[Detection.BINARY_DIFFERENCE] | 
    stronger_kill_adjustment
)
print(rustsmith_rr_adjusted_strong_kills_df.sum(axis=0))


# # Stronger kills than OTS

# In[13]:


print(results_harness_df)


# In[14]:


# OTS unkillable ones
# Or stronger than binary_diff
reliable_strong_kills_idx = rustsmith_rr_strong_df[[Detection.OUTPUT_ERRORS, Detection.OUTPUT_DIFFERENCE]].any(axis=1)
reliably_stronger_than_bindiff = reliable_strong_kills_idx[reliable_strong_kills_idx].index
overall_stronger_kills = rustsmith_not_ots | set(reliably_stronger_than_bindiff)

stronger_kills_df = rustsmith_rr_times_df.loc[overall_stronger_kills, :].drop(columns=[Detection.OUTPUT_TIMEOUT, Detection.COMPILE_TIMEOUT])

# add kill strength of OTS into the table too
stronger_kills_mutants = stronger_kills_df.index
stronger_kills_df["OOTS_Detection"] = [
    (Detection.BINARY_DIFFERENCE if status == "FAILED" else Detection.UNDETECTED) 
    for status in results_harness_df.loc[stronger_kills_mutants, :].STATUS]
print(stronger_kills_df.sort_index())