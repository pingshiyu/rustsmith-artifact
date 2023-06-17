FROM rust:1.67 AS base

# ########################## INSTALL CONDA 
# Install base utilities for Conda, LLVM build, Java, and Creduce
RUN apt-get update \
    && apt-get install -y build-essential \
    && apt-get install -y wget \
    && apt-get install -y cmake \
    && apt-get install -y ninja-build \
    && apt-get install -y openjdk-11-jre-headless \ 
    && apt-get install -y creduce \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install miniconda 
ENV CONDA_DIR=/opt/conda
RUN wget --quiet https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O ~/miniconda.sh && \
    /bin/bash ~/miniconda.sh -b -p /opt/conda

# Put conda in path so we can use conda activate
ENV PATH=$CONDA_DIR/bin:$PATH

# ########################## INSTALL RUSTC FOR THE HISTORICAL BUGS 
RUN rustup install 1.45.0 \
    && rustup install 1.40.0 \
    && rustup install 1.61.0

# ########################## COPY CONDA ENV FILE 
WORKDIR /app

COPY mutation-coverage/environment.yml mutation-coverage/environment.yml

# ########################## SETUP PYTHON ENVIRONMENT (for mutation-coverage) 
RUN conda env create -f mutation-coverage/environment.yml

# ########################## COPY OVER ALL FILES 
COPY . .

# ########################## BUILD RUSTC 
# FROM base AS rustc_building

# build mutated rustc (within mutated-rustc/build)
# FROM rustc_building AS mutated_rustc_build
#### WORKDIR /app/mutation-coverage
#### ARG RUSTC_MUTATION_NUMBER=0
#### RUN ./x.py build -j 4 
# NOTE TO SELF: 4 cores seem to work for the build 

# build coverage-instrumented rustc (within code-coverage/build)
# FROM rustc_building AS coverage_rustc_build
#### WORKDIR /app/code-coverage
#### RUN ./x.py build -j 4

# ########################## EXTRACT BUILT RUSTC
# FROM base AS rustc_built
# COPY --from=mutated_rustc_build /app/mutation-coverage/build /app/mutation-coverage/build
# COPY --from=coverage_rustc_build /app/code-coverage/build /app/code-coverage/build

WORKDIR /app

# for historical bugs replication, we need image with rustup

# NOTE TO SELF test these first, afterwards move them to the end vvvv
# set up python environment (dependencies)
# conda env create -f environment.yml

# chmod +x rustsmith (/home/jacob/projects/rustsmith/rustsmith-artifact/rustsmith/bin/rustsmith)

# set up the two Rust repos: code-coverage and mutation-coverage
# by running ./x.py test test/mir-opt --force-rerun
# which triggers the build.
# swap the order of the two repos so that we don't wait until the whole build is finished to see the results