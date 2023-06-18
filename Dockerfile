FROM rust:1.67 AS base

# Install base utilities for Conda, LLVM build, Java, and Creduce
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    build-essential \
    wget \
    cmake \
    ninja-build \
    openjdk-11-jre-headless \ 
    creduce \
    vim \
    python3-gdbm \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install miniconda (python), needed to build Rustc and for running experiments
ENV CONDA_DIR=/opt/conda
RUN wget --quiet https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O ~/miniconda.sh && \
    /bin/bash ~/miniconda.sh -b -p /opt/conda

# Put conda in path so we can use conda activate
ENV PATH=$CONDA_DIR/bin:$PATH

# mutated-rustc and code-coverage are git submodules. copy .git data into container before building
# this is required by the rustc build pipeline
COPY .git /app/.git
COPY .gitmodules /app/.gitmodules

# Building Rustc for mutation coverage
COPY mutated-rustc /app/mutated-rustc
WORKDIR /app/mutated-rustc
ARG RUSTC_MUTATION_NUMBER=0
RUN ./x.py build -j 8 \
    && ./x.py test src/test/mir-opt --force-rerun

# Building Rustc for code coverage and prepare code coverage tool (grcov)
COPY code-coverage /app/code-coverage
WORKDIR /app/code-coverage
RUN cargo install grcov \
    && ./x.py build -j 8 \
    && ./x.py test src/test/mir-opt --force-rerun

# Most time consuming parts are done, now moving onto faster and/or less stable steps
WORKDIR /app

# Setup python environment for mutation-coverage. The `cp` is a hack to get dbm.gnu working https://stackoverflow.com/a/49597001/2938375 
COPY mutation-coverage/environment.yml /app/mutation-coverage/environment.yml
RUN conda install -n base -c conda-forge mamba \
    && mamba env create -f mutation-coverage/environment.yml \
    && echo "source activate rustsmith-artifact" > ~/.bashrc \
    && cp /usr/lib/python3.9/lib-dynload/_gdbm.cpython-39-x86_64-linux-gnu.so /opt/conda/envs/rustsmith-artifact/lib/python3.9/lib-dynload
ENV PATH /opt/conda/envs/rustsmith-artifact/bin:$PATH

# Installing Rustc for historical bugs
RUN rustup install 1.45.0 \
    && rustup install 1.40.0 \
    && rustup install 1.61.0

# Copy the rest of the files into the container
COPY . .