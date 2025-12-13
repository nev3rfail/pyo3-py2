import pytest


def test_counter_basic():
    """Test basic counter functionality"""
    import class_example

    c = class_example.Counter("test")
    assert c.count == 0
    assert c.name == "test"

    # Test increment
    assert c.increment() == 1
    assert c.count == 1

    assert c.increment(5) == 6
    assert c.count == 6

    # Test decrement
    assert c.decrement() == 5
    assert c.count == 5

    assert c.decrement(3) == 2
    assert c.count == 2

    # Test reset
    c.reset()
    assert c.count == 0


def test_counter_properties():
    """Test counter property access"""
    import class_example

    c = class_example.Counter("props")

    # Test getter
    assert c.name == "props"

    # Test count property
    c.count = 42
    assert c.count == 42


def test_counter_classmethod():
    """Test counter class method"""
    import class_example

    c = class_example.Counter.with_value("initialized", 100)
    assert c.count == 100
    assert c.name == "initialized"


def test_counter_staticmethod():
    """Test counter static method"""
    import class_example

    max_val = class_example.Counter.max_value()
    assert max_val == 2147483647  # i32::MAX


def test_counter_repr():
    """Test counter string representations"""
    import class_example

    c = class_example.Counter("repr_test")
    c.count = 10

    assert repr(c) == "Counter(name='repr_test', count=10)"
    assert str(c) == "repr_test: 10"


def test_point_basic():
    """Test basic point functionality"""
    import class_example

    p = class_example.Point(3.0, 4.0)
    assert p.x == 3.0
    assert p.y == 4.0

    # Distance from origin should be 5.0
    assert abs(p.distance_from_origin() - 5.0) < 0.0001


def test_point_distance():
    """Test point distance calculation"""
    import class_example

    p1 = class_example.Point(0.0, 0.0)
    p2 = class_example.Point(3.0, 4.0)

    assert abs(p1.distance_to(p2) - 5.0) < 0.0001
    assert abs(p2.distance_to(p1) - 5.0) < 0.0001


def test_point_comparison():
    """Test point comparison operators"""
    import class_example

    p1 = class_example.Point(3.0, 4.0)  # distance = 5
    p2 = class_example.Point(6.0, 8.0)  # distance = 10
    p3 = class_example.Point(3.0, 4.0)  # same as p1

    # Test equality
    assert p1 == p3
    assert not (p1 == p2)

    # Test inequality
    assert p1 != p2
    assert not (p1 != p3)

    # Test less than
    assert p1 < p2
    assert not (p2 < p1)

    # Test greater than
    assert p2 > p1
    assert not (p1 > p2)


def test_point_repr():
    """Test point string representation"""
    import class_example

    p = class_example.Point(1.5, 2.5)
    assert repr(p) == "Point(1.5, 2.5)"


def test_sequence_iterator():
    """Test sequence iterator"""
    import class_example

    seq = class_example.Sequence(0, 5)
    result = list(seq)

    assert result == [0, 1, 2, 3, 4]


def test_sequence_multiple_iterations():
    """Test that sequence can be iterated multiple times"""
    import class_example

    seq = class_example.Sequence(1, 4)

    result1 = list(seq)
    result2 = list(seq)

    assert result1 == [1, 2, 3]
    assert result2 == [1, 2, 3]


def test_sequence_empty():
    """Test empty sequence"""
    import class_example

    seq = class_example.Sequence(5, 5)
    result = list(seq)

    assert result == []


if __name__ == "__main__":
    import sys
    sys.path.append(".")

    # Run a quick demo
    import class_example

    print("=== Counter Demo ===")
    c = class_example.Counter("demo")
    print(c)
    c.increment(10)
    print(c)

    print("\n=== Point Demo ===")
    p1 = class_example.Point(3.0, 4.0)
    p2 = class_example.Point(6.0, 8.0)
    print(repr(p1))
    print(repr(p2))
    print("Distance:", p1.distance_to(p2))
    print("p1 < p2:", p1 < p2)

    print("\n=== Sequence Demo ===")
    seq = class_example.Sequence(0, 5)
    print("Sequence:", list(seq))
