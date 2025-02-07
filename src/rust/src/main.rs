use domain::element::{compute_sum_formula, compute_mass, Element, ElementTable};
// use std::io;
pub mod domain;

fn main() {
    let mut _et:ElementTable = ElementTable::new();
    _et.insert(Element::C, 2);
    _et.add(Element::C, 3);
    _et.insert(Element::H, 6);
    _et.insert(Element::O, 1);

    // print the element table
    let sum_formula = compute_sum_formula(&_et);
    let mass = compute_mass(&_et);

    println!("Sum formula: {}", sum_formula);
    println!("Mass: {}", mass);
}
