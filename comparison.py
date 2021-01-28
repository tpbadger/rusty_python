import string
import random
import myrustlib

def count_doubles(chars):
    """
    Count how many duplicate characters there are in a given string
    """
    total = 0 
    for char_1, char_2 in zip(chars, chars[1:]):
        if char_1 == char_2:
            total += 1
    return total


# Do the benchmarks
chars = "".join(random.choice(string.ascii_letters) for i in range(1000000)) # make the string a decent size

def test_pure_python(benchmark):
    benchmark(count_doubles, chars)


def test_pure_rust(benchmark):
    benchmark(myrustlib.count_doubles, chars)

