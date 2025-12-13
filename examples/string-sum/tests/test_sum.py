import pytest
from string_sum import sum_as_string

def test_sum():
    assert sum_as_string(123, 456) == "579"