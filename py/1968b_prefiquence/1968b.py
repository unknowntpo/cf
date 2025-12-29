from dataclasses import dataclass
from pathlib import Path

DEBUG = 1

# https://codeforces.com/problemset/problem/1968/B
"""
10011
1110
"""


@dataclass
class TestCase:
    a: str
    b: str
    expect: int
    actual: int | None = None


def fn(a: str, b: str):
    l = 0
    r = 1
    while r <= len(a):
        substr = a[l:r]
        # TODO: impl

    return 0


def print_result(t: TestCase):
    if DEBUG == 1 and t.expect != t.actual:
        print(f"a: {t.a}, b: {t.b}, expect: {t.expect}, actual: {t.actual}")
    else:
        print(t.actual)


def parse_testcases() -> list[TestCase]:
    with open("1968b_prefiquence/testcases.txt", "r") as f:
        line = f.readline()
        testcases_num = int(line)

        assert testcases_num > 0
        testcases: list[TestCase] = []

        for i in range(testcases_num):
            len_a_b = f.readline()
            a = f.readline()
            b = f.readline()
            # FIXME: deal with expect
            testcases.append(TestCase(a=a, b=b, expect=-1))

        print(testcases)

        return testcases


def main():
    test_cases = parse_testcases()
    print(len(test_cases))
    for t in test_cases:
        t.actual = fn(t.a, t.b)
        print_result(t)


if __name__ == "__main__":
    main()
