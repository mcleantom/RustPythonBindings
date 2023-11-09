from RustPythonBindings import search, search_sequential, search_sequential_allow_threads
from concurrent.futures import ThreadPoolExecutor
from time import perf_counter
from contextlib import contextmanager

@contextmanager
def timer(name: str):
    start = perf_counter()
    yield
    end = perf_counter()
    print(f"{name}: {end - start:.3f}s")


def contents() -> str:
    text = """
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!
"""
    return text * 1000


def run_n_times(executor: ThreadPoolExecutor, func, n: int, *args, **kwargs):
    futures = [executor.submit(func, *args, **kwargs) for _ in range(n)]
    results = [f.result() for f in futures]
    return results


def main():
    words = contents()
    needle = "is"
    n = 10_000
    with ThreadPoolExecutor(max_workers=None) as e:
        with timer("sequential"):
            run_n_times(e, search_sequential, n, words, needle)
        with timer("parallel"):
            run_n_times(e, search, n, words, needle)
        with timer("sequential_allow_threads"):
            run_n_times(e, search_sequential_allow_threads, n, words, needle)


if __name__ == "__main__":
    main()        
