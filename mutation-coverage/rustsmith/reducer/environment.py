from dataclasses import dataclass
from typing import Dict, Optional
from pathlib import Path
import shutil
import pprint as pp

from settings import COMPILE_TIMEOUT, OUTPUT_TIMEOUT
from utils import random_str

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

@dataclass
class ReductionEnv:
    # return values from bug kills
    kill_return_values: Dict[str, int]
    # where to put the reduction folders
    reduction_root: Path = Path("rustsmith/reducer/_reduce")
    # template to generate interesting.sh script
    creduce_script_template_path: Path = Path("rustsmith/reducer/mutation_detection.sh")

def _create_reduce_folder(test_case: TestCase, folder_root: Path, retries : int = 3, verbose: bool = False) -> Path:
    """
    Creates the reduce folder for the test case in `test_case_path`
    """
    # test_case_name = [filename].rs
    test_case_name = test_case.path.name
    # folder_name = [filename]_hash_str
    folder_name = f"{test_case_name.rsplit('.', maxsplit=1)[0]}_{random_str()}"

    # make root folder
    reduce_folder = folder_root / folder_name
    try: 
        if verbose: print("folder created:", reduce_folder)
        reduce_folder.mkdir(parents=True, exist_ok=False)
    except FileExistsError:
        print("folder name collision:", reduce_folder)
        if retries > 0: 
            _create_reduce_folder(test_case, folder_root, retries - 1) # try again

    return reduce_folder

def prepare_reduce_folder(
    test_case: TestCase, 
    environment: ReductionEnv
) -> Path:
    """
    Creates a folder in `folder_root` for `test_case`. 
    The folder will have name `<test_case.path.name>_<salt>`.
    A tempalate `creduce_script_template_path` will be used to generate the 'interestingness' test script.
    Created structure:

    folder_root/
        interesting.sh        # the interestingness test
        test_case.rs          # the test case
        test-case-info.txt    # test case information
    """
    # create and populate folder with test_case 
    reduce_folder = _create_reduce_folder(test_case, environment.reduction_root)
    test_case_path = (reduce_folder / "test-case.rs")
    shutil.copy(test_case.path, test_case_path)
    
    # test_case information (for tracing back on the test_case)
    with open(reduce_folder/"test-case-info.txt", "w") as f:
        f.write(pp.pformat(test_case))

    # create creduce interestingness script
    arguments = ""
    if test_case.cli_args_path:
        arguments = test_case.cli_args_path.open().read().strip()

    script = ""
    with environment.creduce_script_template_path.open() as f:
        script = f.read().format(
            arguments           = arguments,
            rustc_v1            = test_case.v1_config.version,
            flags_1             = test_case.v1_config.flags,
            mutation_1          = test_case.v1_config.mutation,
            rustc_v2            = test_case.v2_config.version,
            flags_2             = test_case.v2_config.flags,
            mutation_2          = test_case.v2_config.mutation,
            time_limit_compile  = test_case.time_limit_compile,
            time_limit_bin      = test_case.time_limit_bin,
            rustc_binary        = test_case.v1_config.compiler_path,
            rs_filename         = test_case_path.name,
            **environment.kill_return_values
        )
        assert test_case.v1_config.compiler_path == test_case.v2_config.compiler_path
    
    # write script to test_case folder
    script_path = reduce_folder / "interesting.sh"
    script_path.touch()
    script_path.write_text(script)
    script_path.chmod(script_path.stat().st_mode | 0o111) # chmod +x

    return reduce_folder