from dataclasses import dataclass
from typing import Optional
from pathlib import Path
from warnings import warn
from multiprocessing import Pool
import subprocess
import shutil

from settings import Detection, MIR_COMPILE_FLAGS, get_kill_return_values, return_code_to_detection, COMPILE_TIMEOUT, OUTPUT_TIMEOUT
from reducer import ReductionEnv, prepare_reduce_folder


@dataclass
class CompilerConfig:
    version: str
    flags: str
    mutation: int = 0
    compiler_path: str = "rustc"

@dataclass
class TestCase:
    v1_config: CompilerConfig
    v2_config: CompilerConfig
    path: Path
    cli_args_path: Optional[Path] = None # default: no args
    time_limit_compile: float = COMPILE_TIMEOUT
    time_limit_bin: float = OUTPUT_TIMEOUT

@dataclass(frozen=True, eq=True)
class MutationContext:
    compiler: str
    mutant: int
    input_path: Path
    input_args_path: Optional[Path]
    reduce_root: Path
    template_script_path: Path
    keep_folder: bool
    panic_kill_is_interesting: bool
    bin_diff_is_interesting: bool
    output_error_is_interesting: bool

def difference_detected(
    test_case: TestCase,
    env: ReductionEnv,
    keep: bool = False,
    timeout: int = 30) -> Detection:
    """
    Checks if the test_case is able to detect the `mutant`. Returns true iff difference exists
    """
    reduction_folder = prepare_reduce_folder(test_case, env)

    # run the interestingness script to check if test_case exists
    try:
        result = subprocess.run(
            "./interesting.sh", cwd=reduction_folder, 
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            timeout=timeout
        )
        return return_code_to_detection(result.returncode)
    except subprocess.TimeoutExpired:
        return Detection.COMPILE_TIMEOUT
    finally:
        # cleanup folder created
        if not keep:
            shutil.rmtree(reduction_folder)

def check_single(env: MutationContext) -> Detection:
    # create a test case using the test script, compiler, and mutation settings.
    test_case = TestCase(
        CompilerConfig("", MIR_COMPILE_FLAGS,          0, env.compiler), 
        CompilerConfig("", MIR_COMPILE_FLAGS, env.mutant, env.compiler), 
        env.input_path, 
        env.input_args_path,
    )

    return_values = get_kill_return_values()
    if env.panic_kill_is_interesting:
        return_values["compile_panic_return"] = 0
        return_values["compile_timeout_return"] = 0
    if env.bin_diff_is_interesting:
        return_values["binary_difference_return"] = 0
    if env.output_error_is_interesting:
        return_values["output_timeout_return"] = 0
        return_values["output_err_return"] = 0
    # default: output_diff_return is interesting
    
    return difference_detected(
        test_case, 
        ReductionEnv(return_values, env.reduce_root, env.template_script_path), 
        keep=env.keep_folder
    )

def check_all(
    contexts: set[MutationContext], 
    jobs: int = 8,
    timeout_early_stop_pct: float = 0.75,
    timeout_check_from: int = 20) -> dict[MutationContext, Detection]:
    """
    Evaluate each context in `contexts` for how strongly the mutant is killed.
    Returns the result of the test. Optionally stop early if the majority of the tests
    were found to be of type `Detection.COMPILE_TIMEOUT`
    """
    test_results = {}
    with Pool(processes=jobs) as p:
        async_results = [p.apply_async(check_single, [context]) for context in contexts]

        # keep running tally of % of failed cases. stop early if we have too many timeouts
        n_timeouts = 0
        n_evaluated = 0

        # get the results       
        for context, result in zip(contexts, async_results):
            n_evaluated += 1
            detected = result.get()
            test_results[context] = detected

            if detected == Detection.UNKNOWN:
                warn(f"Unknown result from: {context}")

            if (detected == Detection.COMPILE_TIMEOUT):
                n_timeouts += 1
                
                # calculate % of failed here. stop early if greater % than threshold
                timeout_rate = n_timeouts / n_evaluated
                if (n_evaluated > timeout_check_from) and (timeout_rate > timeout_early_stop_pct):
                    test_results[context] = Detection.COMPILE_TIMEOUT_STOPPED_EARLY # indicate early stop
                    warn(f"Stopping early on {n_evaluated} due to timeout rate of {timeout_rate*100}%: {context}")
                    break
    return test_results