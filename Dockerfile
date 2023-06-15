# for historical bugs replication, we need image with rustup

# set up the two Rust repos: code-coverage and mutation-coverage
# by running ./x.py test test/mir-opt --force-rerun
# which triggers the build.
# swap the order of the two repos so that we don't wait until the whole build is finished to see the results

# chmod +x rustsmith (/home/jacob/projects/rustsmith/rustsmith-artifact/rustsmith/bin/rustsmith)

# set up python environment (dependencies)