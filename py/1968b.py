from dataclasses import dataclass

# https://codeforces.com/problemset/problem/1968/B

input = """
6
5 4
10011
1110
3 3
100
110
1 3
1
111
4 4
1011
1111
3 5
100
11010
3 1
100
0
"""


@dataclass
class TestCase:
    a: str
    b: str
    expect: int
    actual: int | None = None


def fn(a: str, b: str):
    return 0


def print_result(t: TestCase):
    if t.expect != t.actual:
        print(f"a: {t.a}, b: {t.b}, expect: {t.expect}, actual: {t.actual}")


def parse_testcases(input: str) -> list[TestCase]:
    return [TestCase(a="001", b="110", expect=3)]


def main():
    test_cases = parse_testcases(input)
    print(len(test_cases))
    for t in test_cases:
        t.actual = fn(t.a, t.b)
        print_result(t)


if __name__ == "__main__":
    main()
