from pyfib.main import get_fibonacci, get_fibonacci_rust


def test_python_fib(benchmark):
    benchmark(get_fibonacci, 100)


def test_rust_fib(benchmark):
    benchmark(get_fibonacci_rust, 100)
