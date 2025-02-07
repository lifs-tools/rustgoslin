use extendr_api::{prelude::*};

use domain::element::{ElementTable};
use extendr_api::Robj;
use std::{collections::HashMap, convert::TryFrom};

impl TryFrom<Robj> for ElementTable {
    type Error = extendr_api::Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        //require a named list
        if !value.is_list() {
            return Err(extendr_api::Error::from("Expected a named list"));
        }
        // use the named list to create an ElementTable,
        // names are the element symbols and values are the counts
        let mut et = ElementTable::new();
        let element_list = value.as_list().unwrap();
        element_list.iter().for_each(|i| {
            let name = i.0;
            let count = i.1.as_integer().unwrap();
            let element = domain::element::Element::from(name.to_string());
            et.insert(element, count);
        });
        Ok(et)
    }
}

// include domain module
mod domain;

/// Return sum formula for provided element table.
/// @export
#[extendr]
fn compute_sum_formula(et: List) -> String {
    let et_hashmap: HashMap<&str, Robj> = et.into_hashmap();
    println!("Element Table: {:?}", et_hashmap);
    let mut et = ElementTable::new();
    et_hashmap.iter().for_each(|(k, v)| {
        rprintln!("v has an Rtype of {:?}", v.rtype());
        rprintln!("{:?}", Integers::try_from(&v));
        let element = domain::element::Element::from(k.to_string());
        // print element, count
        println!("{:?}: {}", element, v.as_integer().unwrap());
        let count = v.as_integer().unwrap();
        et.insert(element, count);
    });
    let sf = domain::element::compute_sum_formula(&et);
    sf
}

/// Return mass for provided element table.
/// @export
#[extendr]
fn compute_mass(et: List) -> f64 {
    let et_hashmap: HashMap<&str, Robj> = et.into_hashmap();
    let mut et = ElementTable::new();
    et_hashmap.iter().for_each(|(k, v)| {
        let element = domain::element::Element::from(k.to_string());
        // print element, count
        println!("{}: {}", element, v.as_integer().unwrap());
        let count = v.as_integer().unwrap();
        et.insert(element, count);
    });
    domain::element::compute_mass(&et)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustgoslin;
    fn compute_sum_formula;
    fn compute_mass;
}
