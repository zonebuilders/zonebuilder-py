from zonebuilder import clockboard, triangular_sequence
import geojson, pytest


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
def test_triangular_sequence(test_io):
    output = triangular_sequence(test_io["input"])
    assert test_io["output"] == output


def test_default():
    featurecollection = geojson.loads(clockboard([0, 0]))
    assert len(featurecollection["features"]) == 49


@pytest.mark.parametrize(
    "test_io",
    [
        # case 1
        {"num_segments": 11, "distances": triangular_sequence(10)},
        # case 2
        {"num_segments": 1, "distances": triangular_sequence(3)},
        # case 3
        {"num_segments": 4, "distances": triangular_sequence(1)},
    ],
)
def test_n_polygons(test_io):
    featurecollection = geojson.loads(
        clockboard(
            [0, 0], num_segments=test_io["num_segments"], distances=test_io["distances"]
        )
    )
    if test_io["num_segments"] > 1:
        assert len(featurecollection["features"]) == (
            test_io["num_segments"] * (len(test_io["distances"]) - 1) + 1
        )
    else:
        assert len(featurecollection["features"]) == len(test_io["distances"])
