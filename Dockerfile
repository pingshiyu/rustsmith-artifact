FROM rust:1.67 AS base

# Install base utilities for Conda, LLVM build, Java, and Creduce
RUN apt-get update \
    && apt-get install -y build-essential \
    && apt-get install -y wget \
    && apt-get install -y cmake \
    && apt-get install -y ninja-build \
    && apt-get install -y openjdk-11-jre-headless \ 
    && apt-get install -y creduce \
    && apt-get install -y vim \
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

# Setup python environment for mutation-coverage
COPY mutation-coverage/environment.yml /app/mutation-coverage/environment.yml
RUN conda env create -f mutation-coverage/environment.yml

# Installing Rustc for historical bugs
RUN rustup install 1.45.0 \
    && rustup install 1.40.0 \
    && rustup install 1.61.0

# Copy the rest of the files into the container
COPY . .

# Use the conda environment for container
SHELL ["conda", "run", "-n", "rustsmith-artifact", "/bin/bash", "-c"]