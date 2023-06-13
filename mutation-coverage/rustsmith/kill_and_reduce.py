"""
cmd line interface to check if the {rust-script, compilers} test case demonstrates mutation-differential on mutation x.
"""
from settings import MUTATED_RUSTC_PATH, MAX_MUTANT, DEFAULT_REDUCE_ROOT, TEMPLATE_SCRIPT_PATH, Detection
from coverage import check_all, check_single, MutationContext

from pathlib import Path
import argparse

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="python -m rustsmith.kill_and_reduce",
        description="Kill mutants using randomly generated programs as test cases, and reduce the generated test case."
    )

    test_case_config = parser.add_argument_group('Test Case Specification')
    test_case_config.add_argument("-i", "--input-path", type=Path, required=True,
                                  help="Path input script triggering the test_case.")
    test_case_config.add_argument("--no-input-args", action="store_true",
                                  help="This file does not have any input args.")
    test_case_config.add_argument("-a", "--input-args-path", type=Path, required=False,
                                  help="Path to the cmd args of the input test case.")
    

    compiler_config = test_case_config.add_mutually_exclusive_group()
    compiler_config.add_argument("-c", "--compiler", type=str,
                                 default=MUTATED_RUSTC_PATH,
                                 help="Location of compiler to use")
    compiler_config.add_argument("--use-default-compiler", action="store_const", dest="compiler",
                                 const="rustc",
                                 help="Use the default rustc compiler")
    compiler_config.add_argument("--use-mutation-compiler", action="store_const", dest="compiler",
                                 const=MUTATED_RUSTC_PATH,
                                 help="Use the rustc compiler with mutations built in")

    mutation_config = parser.add_argument_group("Mutation Settings")
    mutation_config.add_argument("-m", "--mutation", type=int, required=True,
                                 help="Type of mutation to compare (against no mutations)")
    mutation_config.add_argument("--try-all", action="store_true", default=False,
                        help="Try all mutatnts in range [mutation, max_mutantion]?")
    mutation_config.add_argument("-mm", "--max-mutation", type=int, default=MAX_MUTANT,
                                 help="Which mutant number to try up to?")
    mutation_config.add_argument("--panic-kill-interesting", action="store_true", default=False,
                                 help="Are panic kills interesting to the reducer?")
    mutation_config.add_argument("--bin-diff-interesting", action="store_true", default=False,
                                 help="Is the binary being different (but same output when ran) interesting to the reducer?")
    mutation_config.add_argument("--output-error-interesting", action="store_true", default=False,
                                 help="Are output binary errors/timeouts interesting to the reducer?")

    parser.add_argument("--reduce-root", type=Path, required=False,
                        default=DEFAULT_REDUCE_ROOT,
                        help="Place to put the reduce folder")
    parser.add_argument("--template-script", type=Path, required=False,
                        default=TEMPLATE_SCRIPT_PATH,
                        help="Place to put the reduce folder")
    parser.add_argument("--keep", action="store_true", default=False,
                        help="Keep created temp reduction folder")
    

    # do a bit more parsing once inputs are specified
    args = parser.parse_args()

    # default: if input-args doesn't exist, and input args are expected, assume it's the .txt 
    # file with the same name as `input_path`
    if (not args.no_input_args) and (not args.input_args_path):
        args.input_args_path = get_default_args_path(args.input_path)

    return args

def get_default_args_path(test_path: Path) -> Path:
    test_name = test_path.name.rsplit('.', maxsplit=1)[0]
    return test_path.parent / f"{test_name}.txt"

def _get_contexts(args: argparse.Namespace) -> set[MutationContext] | MutationContext:
    """
    Returns a list of contexts iff `args.try_all == True`.
    """
    if args.try_all:
        contexts = set()
        for m in range(args.mutation, args.max_mutation):
            context = MutationContext(args.compiler, m, args.input_path, args.input_args_path,
                                  args.reduce_root, args.template_script, args.keep,
                                  args.panic_kill_interesting, args.bin_diff_interesting, args.output_error_interesting)
            contexts.add(context)
        return contexts
    else:
        return MutationContext(args.compiler, args.mutation, args.input_path, args.input_args_path,
                           args.reduce_root, args.template_script, args.keep,
                           args.panic_kill_interesting, args.bin_diff_interesting, args.output_error_interesting)

def main() -> None:
    args = parse_args()
    # print(args)

    envs = _get_contexts(args)

    if args.try_all:
        results = check_all(envs)
        mutation_results = [(mut_test.mutant, detection) for mut_test, detection in results.items()
                            if (detection != Detection.UNDETECTED) and (detection != Detection.UNKNOWN)]
        print(f"mutants detected by {args.input_path}:", mutation_results)
    else:
        print(f"mutant {args.mutation} detected by {args.input_path}:", check_single(envs))

if __name__ == '__main__':
    main()
