import zonebuilder
import pytest


@pytest.mark.parametrize(
    "test_io",
    [
        # case 1
        {"input": 1, "output": [1.0]},
        # case 2
        {"input": 2, "output": [1.0, 3.0]},
        # case 3
        {"input": 3, "output": [1.0, 3.0, 6.0]},
        # case 4
        {"input": 4, "output": [1.0, 3.0, 6.0, 10.0]},
    ],
)
def test_triangle(test_io):
    output = zonebuilder.triangular_sequence(test_io["input"])
    assert test_io["output"] == output
