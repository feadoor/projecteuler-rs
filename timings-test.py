import os
import sys
import re
from timeit import timeit

def main():
    if len(sys.argv) < 2:
        print "Usage: python timings-test.py <path-to-build-output-dir>"
    else:
        output_dir = sys.argv[1]
        src_files = reduce(lambda x, y: x + y, map(lambda x: x[2], os.walk('src')))
        problem_regex = re.compile("problem(\d{3})\.rs")
        problem_numbers = map(lambda x: int(x.group(1)), filter(lambda x: x is not None, [problem_regex.match(filename) for filename in src_files]))

        failed_problems = []
        for problem_number in problem_numbers:
            print "Problem {0:03d}:".format(problem_number),
            executable = os.path.join(output_dir, "p{0:03d}".format(problem_number))
            time = timeit(stmt="subprocess.call(['{0}'], stdout=open(os.devnull, 'w'))".format(executable), setup="import os, subprocess", number=1)
            print "{0:.2f} seconds".format(time)

            if time > 60:
                failed_problems.append(problem_number)

        if failed_problems:
            print "Execution time limit exceeded for problems {0}".format(failed_problems)
            sys.exit(1)

if __name__ == "__main__":
    main()