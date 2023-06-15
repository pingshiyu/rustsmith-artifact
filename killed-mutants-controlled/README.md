# Mutation Testing Controlled Experiment (Results)

This folder contains the results quoted for the mutations controlled experiment, described in the paper.

Folders are named after the mutant IDs. If a RustSmith generated file kills the mutant, then the file is saved within a folder named `<KILL_REASON>_<hash>`. Each folder also contains an `info.json` file storing the metadata for the mutant's 3-minute killing round - the type of kill, and the time taken to find the kill within the experiment.