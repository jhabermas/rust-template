from hello import get_fibonacci_rust

def get_fibonacci(number: int) -> int:
    """Get the nth Fibonacci number."""
    if number == 1:
        return 1
    elif number == 2:
        return 2

    total = 0
    last = 0
    current = 1
    for _ in range(1, number):
        total = last + current
        last = current
        current = total
    return total


if __name__ == '__main__':
    print(get_fibonacci(10))
    print(get_fibonacci_rust(10))
