# Example: Simple Python function to be converted

def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def sum_list(numbers: list) -> int:
    """Sum all numbers in a list."""
    total = 0
    for num in numbers:
        total += num
    return total


def filter_even(numbers: list) -> list:
    """Filter even numbers from a list."""
    return [n for n in numbers if n % 2 == 0]


if __name__ == "__main__":
    print(f"Fibonacci(10): {fibonacci(10)}")
    print(f"Sum: {sum_list([1, 2, 3, 4, 5])}")
    print(f"Even numbers: {filter_even([1, 2, 3, 4, 5, 6])}")
