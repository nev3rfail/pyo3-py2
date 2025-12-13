
def test_sum():
    import string_sum
    assert string_sum.sum_as_string(123, 456) == "579"

if __name__ == "__main__":
    import sys
    sys.path.append(".")
    import string_sum
    sum = string_sum.sum_as_string(123, 456)
    print(type(sum), sum)