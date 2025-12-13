# Class Example

This example demonstrates various PyO3 class features for Python 2, including:

## Features Demonstrated

### 1. Counter Class
- **Constructor** (`__new__`): Creating instances from Python
- **Properties**: Using `#[prop(get, set)]` for simple property access
- **Getters**: Custom getter methods with `#[getter]`
- **Instance methods**: Methods that modify state
- **Method arguments**: Default arguments using `#[args()]`
- **Class methods**: Factory methods with `#[classmethod]`
- **Static methods**: Utility methods with `#[staticmethod]`
- **String protocols**: `__repr__` and `__str__` implementations

### 2. Point Class
- **Read-only properties**: Getters without setters
- **Comparison operators**: `__richcmp__` for implementing `<`, `<=`, `==`, `!=`, `>`, `>=`
- **Instance methods**: Methods that work with other instances

### 3. Sequence Class
- **Iterator protocol**: `__iter__` and `__next__` implementations
- **Stateful iteration**: Managing iteration state

## Building

```bash
pip install -r requirements-dev.txt
python setup.py develop
```

## Testing

```bash
pytest tests/
```

Or with tox:
```bash
tox
```

## Usage Examples

```python
import class_example

# Counter example
counter = class_example.Counter("my_counter")
print(counter)  # my_counter: 0

counter.increment(5)
print(counter.count)  # 5

counter.reset()
print(repr(counter))  # Counter(name='my_counter', count=0)

# Create counter with initial value
c2 = class_example.Counter.with_value("initialized", 100)
print(c2.count)  # 100

# Point example
p1 = class_example.Point(3.0, 4.0)
p2 = class_example.Point(6.0, 8.0)

print(p1.distance_from_origin())  # 5.0
print(p1.distance_to(p2))  # 5.0

print(p1 < p2)  # True (compared by distance from origin)

# Sequence example
seq = class_example.Sequence(0, 5)
for i in seq:
    print(i)  # Prints 0, 1, 2, 3, 4
```

## Guide Reference

This example is based on the [Python Class guide](../../guide/src/class.md) and demonstrates:
- Defining classes with `#[pyclass]`
- Constructor methods with `#[new]`
- Instance methods with `#[pymethods]`
- Properties with `#[getter]`, `#[setter]`, and `#[prop]`
- Class and static methods
- Protocol implementations with `#[pyproto]`
- Basic object protocol (`__repr__`, `__str__`, `__richcmp__`)
- Iterator protocol (`__iter__`, `__next__`)
