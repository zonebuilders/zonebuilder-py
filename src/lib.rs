use geo::Point;
use pyo3::{prelude::*, types::PyDict, Python};
use zonebuilder::Params;

#[pyfunction]
fn triangular_sequence(n: usize) -> Vec<f64> {
    zonebuilder::triangular_sequence(n)
}

#[pyclass]
#[derive(Clone)]
struct Pyparams {
    num_segments: usize,
    distances: Vec<f64>,
    num_vertices_arc: usize,
    precision: usize,
    projected: bool,
}

impl Default for Pyparams {
    fn default() -> Self {
        let defaults = Params::default();
        Pyparams {
            num_segments: defaults.num_segments,
            distances: defaults.distances,
            num_vertices_arc: defaults.num_vertices_arc,
            precision: defaults.precision,
            projected: defaults.projected,
        }
    }
}

impl Pyparams {
    fn to_zb_params(&self) -> Params {
        Params {
            num_segments: self.num_segments,
            distances: self.distances.clone(),
            num_vertices_arc: self.num_vertices_arc,
            precision: self.precision,
            projected: self.projected,
        }
    }
}

#[pyfunction(kwargs = "**")]
fn kwargsparse(kwargs: Option<&PyDict>) -> PyResult<Pyparams> {
    let defaults = Pyparams::default();
    if kwargs.is_none() {
        return Ok(defaults);
    }
    let udict = kwargs.unwrap();
    let params = Pyparams {
        num_segments: match udict.get_item("num_segments") {
            Some(value) => value.extract()?,
            None => defaults.num_segments,
        },
        distances: match udict.get_item("distances") {
            Some(value) => value.extract()?,
            None => defaults.distances,
        },
        num_vertices_arc: match udict.get_item("num_vertices_arc") {
            Some(value) => value.extract()?,
            None => defaults.num_vertices_arc,
        },
        precision: match udict.get_item("precision") {
            Some(value) => value.extract()?,
            None => defaults.precision,
        },
        projected: match udict.get_item("projected") {
            Some(value) => value.extract()?,
            None => defaults.projected,
        },
    };
    Ok(params)
}

#[pyfunction(kwargs = "**")]
fn clockboard(center: [f64; 2], kwargs: Option<&PyDict>) -> PyResult<String> {
    let center_point = Point::new(center[0], center[1]);
    let params = kwargsparse(kwargs)?;
    let geojson = zonebuilder::clockboard(center_point, params.to_zb_params()).to_string();
    Ok(geojson)
}

#[pymodule]
fn zonebuilder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangular_sequence, m)?)?;
    m.add_function(wrap_pyfunction!(clockboard, m)?)?;
    Ok(())
}
