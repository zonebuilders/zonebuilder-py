# zonebuilder-py
Python package for creating zoning systems

## Dev-notes
### Installation
Steps to test the python bindings for `zonebuilder-py`
1. Create a local virtual environment for python and activate it.
```
python -m venv env
source env/bin/activate
```
2. Install necessary libraries into the environment
```
pip install -r requirements.txt
```
3. Develop the rust code into a python package and install it into the local environment
```
maturin develop
```
alternately you can also
```
pip install .
```
4. Test it (somehow directly typing `pytest` throws error), use below
```
python -m pytest
```

### Current functions
1. `triangular_sequence`
2. `clockboard`