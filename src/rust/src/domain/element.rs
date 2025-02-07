/*
Original C++ Header


#ifndef ELEMENT_H_FILE
#define ELEMENT_H_FILE

#define ELECTRON_REST_MASS 0.00054857990946

#include <string>
#include <map>
#include <vector>
#include <sstream>

namespace goslin {
using namespace std;

enum Element {ELEMENT_C, ELEMENT_C13, ELEMENT_H, ELEMENT_H2, ELEMENT_N, ELEMENT_N15, ELEMENT_O, ELEMENT_O17, ELEMENT_O18, ELEMENT_P, ELEMENT_P32, ELEMENT_S, ELEMENT_S34, ELEMENT_S33, ELEMENT_F, ELEMENT_Cl, ELEMENT_Br, ELEMENT_I, ELEMENT_As};


typedef map<Element, int> ElementTable;


ElementTable* create_empty_table();
string goslin::compute_sum_formula(ElementTable* elements){
    stringstream ss;

    for (auto e : element_order){
        if (elements->at(e) > 0) ss << element_shortcut.at(e);
        if (elements->at(e) > 1) ss << elements->at(e);
    }
    return ss.str();
}

const map<string, Element> element_positions = {{"C", ELEMENT_C}, {"H", ELEMENT_H}, {"N", ELEMENT_N}, {"O", ELEMENT_O}, {"P", ELEMENT_P}, {"P'", ELEMENT_P32}, {"S", ELEMENT_S}, {"F", ELEMENT_F}, {"Cl", ELEMENT_Cl}, {"Br", ELEMENT_Br}, {"I", ELEMENT_I}, {"As", ELEMENT_As}, {"S'", ELEMENT_S34}, {"S''", ELEMENT_S33}, {"H'", ELEMENT_H2}, {"C'", ELEMENT_C13}, {"N'", ELEMENT_N15}, {"O'", ELEMENT_O17}, {"O''", ELEMENT_O18}, {"2H", ELEMENT_H2}, {"13C", ELEMENT_C13}, {"15N", ELEMENT_N15}, {"17O", ELEMENT_O17}, {"18O", ELEMENT_O18}, {"32P", ELEMENT_P32}, {"34S", ELEMENT_S34}, {"33S", ELEMENT_S33}, {"H2", ELEMENT_H2}, {"C13", ELEMENT_C13}, {"N15", ELEMENT_N15}, {"O17", ELEMENT_O17}, {"O18", ELEMENT_O18}, {"P32", ELEMENT_P32}, {"S34", ELEMENT_S34}, {"S33", ELEMENT_S33}};


const map<Element, double> element_masses = {{ELEMENT_C, 12.0},  {ELEMENT_H, 1.007825035},  {ELEMENT_N, 14.0030740}, {ELEMENT_O, 15.99491463}, {ELEMENT_P, 30.973762},  {ELEMENT_S, 31.9720707}, {ELEMENT_H2, 2.014101779},  {ELEMENT_C13, 13.0033548378},  {ELEMENT_N15, 15.0001088984}, {ELEMENT_O17, 16.9991315}, {ELEMENT_O18, 17.9991604}, {ELEMENT_P32, 31.973907274}, {ELEMENT_S33, 32.97145876}, {ELEMENT_S34, 33.96786690}, {ELEMENT_F, 18.9984031}, {ELEMENT_Cl, 34.968853}, {ELEMENT_Br, 78.918327}, {ELEMENT_I, 126.904473}, {ELEMENT_As, 74.921595}};


const map<Element, string> element_shortcut = {{ELEMENT_C, "C"}, {ELEMENT_H, "H"}, {ELEMENT_N, "N"}, {ELEMENT_O, "O"}, {ELEMENT_P, "P"}, {ELEMENT_S, "S"}, {ELEMENT_F, "F"}, {ELEMENT_Cl, "Cl"}, {ELEMENT_Br, "Br"}, {ELEMENT_I, "I"}, {ELEMENT_As, "As"}, {ELEMENT_H2, "H'"}, {ELEMENT_C13, "C'"}, {ELEMENT_N15, "N'"}, {ELEMENT_O17, "O'"}, {ELEMENT_O18, "O''"}, {ELEMENT_P32, "P'"}, {ELEMENT_S33, "S'"}, {ELEMENT_S34, "S''"}};

const map<Element, string> heavy_shortcut = {{ELEMENT_C, "C"}, {ELEMENT_H, "H"}, {ELEMENT_N, "N"}, {ELEMENT_O, "O"}, {ELEMENT_P, "P"}, {ELEMENT_S, "S"}, {ELEMENT_F, "F"}, {ELEMENT_I, "I"}, {ELEMENT_As, "As"}, {ELEMENT_Br, "Br"}, {ELEMENT_Cl, "Cl"}, {ELEMENT_H2, "[2]H"}, {ELEMENT_C13, "[13]C"}, {ELEMENT_N15, "[15]N"}, {ELEMENT_O17, "[17]O"}, {ELEMENT_O18, "[18]O"}, {ELEMENT_P32, "[32]P"}, {ELEMENT_S33, "[33]S"}, {ELEMENT_S34, "[34]S"}};


const map<Element, Element> heavy_to_regular = {{ELEMENT_H2, ELEMENT_H}, {ELEMENT_C13, ELEMENT_C}, {ELEMENT_N15, ELEMENT_N}, {ELEMENT_O17, ELEMENT_O}, {ELEMENT_O18, ELEMENT_O}, {ELEMENT_P32, ELEMENT_P}, {ELEMENT_S33, ELEMENT_S}, {ELEMENT_S34, ELEMENT_S}};


const vector<Element> element_order = {ELEMENT_C, ELEMENT_H, ELEMENT_As, ELEMENT_Br, ELEMENT_Cl, ELEMENT_F, ELEMENT_I, ELEMENT_N, ELEMENT_O, ELEMENT_P, ELEMENT_S, ELEMENT_H2, ELEMENT_C13, ELEMENT_N15, ELEMENT_O17, ELEMENT_O18, ELEMENT_P32, ELEMENT_S33, ELEMENT_S34};


const map<string, Element> heavy_element_table = {{"[2]H", ELEMENT_H2}, {"[13]C", ELEMENT_C13}, {"[15]N", ELEMENT_N15}, {"[17]O", ELEMENT_O17}, {"[18]O", ELEMENT_O18}, {"[32]P", ELEMENT_P32}, {"[33]S", ELEMENT_S33}, {"[34]S", ELEMENT_S34}};

}
#endif /* ELEMENT_H_FILE */
*/

use lazy_static::lazy_static;
use maplit::btreemap;
use std::{collections::BTreeMap, sync::Mutex};

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Element {
    C,
    C13,
    H,
    H2,
    N,
    N15,
    O,
    O17,
    O18,
    P,
    P32,
    S,
    S34,
    S33,
    F,
    Cl,
    Br,
    I,
    As,
    NONE,
}

impl From<Element> for extendr_api::Robj {
    fn from(e: Element) -> Self {
        // convert Element to string
        let binding = ELEMENT_SHORTCUT.try_lock();
        match binding {
            Ok(b) => {
                let s = b.get(&e).unwrap();
                return s.into()
            }
            Err(_) => {
                return extendr_api::Robj::from("Error")
            }
        }
    }
}

impl From<String> for Element {
    fn from(s: String) -> Self {
        let str = s.to_string();
        match str.as_str() {
            "C" => Element::C,
            "C13" => Element::C13,
            "H" => Element::H,
            "H2" => Element::H2,
            "N" => Element::N,
            "N15" => Element::N15,
            "O" => Element::O,
            "O17" => Element::O17,
            "O18" => Element::O18,
            "P" => Element::P,
            "P32" => Element::P32,
            "S" => Element::S,
            "S34" => Element::S34,
            "S33" => Element::S33,
            "F" => Element::F,
            "Cl" => Element::Cl,
            "Br" => Element::Br,
            "I" => Element::I,
            "As" => Element::As,
            _ => Element::NONE,
        }
    }
}

// create ElementTable as a struct inheriting from BTreeMap<Element, i32>
pub struct ElementTable {
    pub table: BTreeMap<Element, i32>,
}

impl ElementTable {
    pub fn new() -> ElementTable {
        ElementTable {
            table: btreemap! {
                Element::C => 0,
                Element::C13 => 0,
                Element::H => 0,
                Element::H2 => 0,
                Element::N => 0,
                Element::N15 => 0,
                Element::O => 0,
                Element::O17 => 0,
                Element::O18 => 0,
                Element::P => 0,
                Element::P32 => 0,
                Element::S => 0,
                Element::S34 => 0,
                Element::S33 => 0,
                Element::F => 0,
                Element::Cl => 0,
                Element::Br => 0,
                Element::I => 0,
                Element::As => 0,
            },
        }
    }
    pub fn get(&self, e: Element) -> i32 {
        *self.table.get(&e).unwrap()
    }

    pub fn insert(&mut self, e: Element, n: i32) {
        self.table.insert(e, n);
    }

    pub fn add(&mut self, e: Element, n: i32) {
        self.table.insert(e, self.get(e) + n);
    }
}

// element_positions
/*
const map<string, Element> element_positions = {{"C", ELEMENT_C}, {"H", ELEMENT_H}, {"N", ELEMENT_N}, {"O", ELEMENT_O}, {"P", ELEMENT_P}, {"P'", ELEMENT_P32}, {"S", ELEMENT_S}, {"F", ELEMENT_F}, {"Cl", ELEMENT_Cl}, {"Br", ELEMENT_Br}, {"I", ELEMENT_I}, {"As", ELEMENT_As}, {"S'", ELEMENT_S34}, {"S''", ELEMENT_S33}, {"H'", ELEMENT_H2}, {"C'", ELEMENT_C13}, {"N'", ELEMENT_N15}, {"O'", ELEMENT_O17}, {"O''", ELEMENT_O18}, {"2H", ELEMENT_H2}, {"13C", ELEMENT_C13}, {"15N", ELEMENT_N15}, {"17O", ELEMENT_O17}, {"18O", ELEMENT_O18}, {"32P", ELEMENT_P32}, {"34S", ELEMENT_S34}, {"33S", ELEMENT_S33}, {"H2", ELEMENT_H2}, {"C13", ELEMENT_C13}, {"N15", ELEMENT_N15}, {"O17", ELEMENT_O17}, {"O18", ELEMENT_O18}, {"P32", ELEMENT_P32}, {"S34", ELEMENT_S34}, {"S33", ELEMENT_S33}};
*/
lazy_static! {
    static ref ELEMENT_POSITIONS: Mutex<BTreeMap<String, Element>> = Mutex::new(btreemap! {
        "C".to_string() => Element::C,
        "H".to_string() => Element::H,
        "N".to_string() => Element::N,
        "O".to_string() => Element::O,
        "P".to_string() => Element::P,
        "P'".to_string() => Element::P32,
        "S".to_string() => Element::S,
        "F".to_string() => Element::F,
        "Cl".to_string() => Element::Cl,
        "Br".to_string() => Element::Br,
        "I".to_string() => Element::I,
        "As".to_string() => Element::As,
        "S'".to_string() => Element::S34,
        "S''".to_string() => Element::S33,
        "H'".to_string() => Element::H2,
        "C'".to_string() => Element::C13,
        "N'".to_string() => Element::N15,
        "O'".to_string() => Element::O17,
        "O''".to_string() => Element::O18,
        "2H".to_string() => Element::H2,
        "13C".to_string() => Element::C13,
        "15N".to_string() => Element::N15,
        "17O".to_string() => Element::O17,
        "18O".to_string() => Element::O18,
        "32P".to_string() => Element::P32,
        "34S".to_string() => Element::S34,
        "33S".to_string() => Element::S33,
        "H2".to_string() => Element::H2,
        "C13".to_string() => Element::C13,
        "N15".to_string() => Element::N15,
        "O17".to_string() => Element::O17,
        "O18".to_string() => Element::O18,
        "P32".to_string() => Element::P32,
        "S34".to_string() => Element::S34,
        "S33".to_string() => Element::S33,
    });
}

// const map<Element, double> element_masses = {{ELEMENT_C, 12.0},  {ELEMENT_H, 1.007825035},  {ELEMENT_N, 14.0030740}, {ELEMENT_O, 15.99491463}, {ELEMENT_P, 30.973762},  {ELEMENT_S, 31.9720707}, {ELEMENT_H2, 2.014101779},  {ELEMENT_C13, 13.0033548378},  {ELEMENT_N15, 15.0001088984}, {ELEMENT_O17, 16.9991315}, {ELEMENT_O18, 17.9991604}, {ELEMENT_P32, 31.973907274}, {ELEMENT_S33, 32.97145876}, {ELEMENT_S34, 33.96786690}, {ELEMENT_F, 18.9984031}, {ELEMENT_Cl, 34.968853}, {ELEMENT_Br, 78.918327}, {ELEMENT_I, 126.904473}, {ELEMENT_As, 74.921595}};
lazy_static! {
    static ref ELEMENT_MASSES: Mutex<BTreeMap<Element, f64>> = Mutex::new(btreemap! {
        Element::C => 12.0,
        Element::H => 1.007825035,
        Element::N => 14.0030740,
        Element::O => 15.99491463,
        Element::P => 30.973762,
        Element::S => 31.9720707,
        Element::H2 => 2.014101779,
        Element::C13 => 13.0033548378,
        Element::N15 => 15.0001088984,
        Element::O17 => 16.9991315,
        Element::O18 => 17.9991604,
        Element::P32 => 31.973907274,
        Element::S33 => 32.97145876,
        Element::S34 => 33.96786690,
        Element::F => 18.9984031,
        Element::Cl => 34.968853,
        Element::Br => 78.918327,
        Element::I => 126.904473,
        Element::As => 74.921595,
    });
}

// const map<Element, string> element_shortcut = {{ELEMENT_C, "C"}, {ELEMENT_H, "H"}, {ELEMENT_N, "N"}, {ELEMENT_O, "O"}, {ELEMENT_P, "P"}, {ELEMENT_S, "S"}, {ELEMENT_F, "F"}, {ELEMENT_Cl, "Cl"}, {ELEMENT_Br, "Br"}, {ELEMENT_I, "I"}, {ELEMENT_As, "As"}, {ELEMENT_H2, "H'"}, {ELEMENT_C13, "C'"}, {ELEMENT_N15, "N'"}, {ELEMENT_O17, "O'"}, {ELEMENT_O18, "O''"}, {ELEMENT_P32, "P'"}, {ELEMENT_S33, "S'"}, {ELEMENT_S34, "S''"}};
lazy_static! {
    static ref ELEMENT_SHORTCUT: Mutex<BTreeMap<Element, String>> = Mutex::new(btreemap! {
        Element::C => "C".to_string(),
        Element::H => "H".to_string(),
        Element::N => "N".to_string(),
        Element::O => "O".to_string(),
        Element::P => "P".to_string(),
        Element::S => "S".to_string(),
        Element::F => "F".to_string(),
        Element::Cl => "Cl".to_string(),
        Element::Br => "Br".to_string(),
        Element::I => "I".to_string(),
        Element::As => "As".to_string(),
        Element::H2 => "H'".to_string(),
        Element::C13 => "C'".to_string(),
        Element::N15 => "N'".to_string(),
        Element::O17 => "O'".to_string(),
        Element::O18 => "O''".to_string(),
        Element::P32 => "P'".to_string(),
        Element::S33 => "S'".to_string(),
        Element::S34 => "S''".to_string(),
    });
}

// const map<Element, string> heavy_shortcut = {{ELEMENT_C, "C"}, {ELEMENT_H, "H"}, {ELEMENT_N, "N"}, {ELEMENT_O, "O"}, {ELEMENT_P, "P"}, {ELEMENT_S, "S"}, {ELEMENT_F, "F"}, {ELEMENT_I, "I"}, {ELEMENT_As, "As"}, {ELEMENT_Br, "Br"}, {ELEMENT_Cl, "Cl"}, {ELEMENT_H2, "[2]H"}, {ELEMENT_C13, "[13]C"}, {ELEMENT_N15, "[15]N"}, {ELEMENT_O17, "[17]O"}, {ELEMENT_O18, "[18]O"}, {ELEMENT_P32, "[32]P"}, {ELEMENT_S33, "[33]S"}, {ELEMENT_S34, "[34]S"}};
lazy_static! {
    static ref HEAVY_SHORTCUT: Mutex<BTreeMap<Element, String>> = Mutex::new(btreemap! {
        Element::C => "C".to_string(),
        Element::H => "H".to_string(),
        Element::N => "N".to_string(),
        Element::O => "O".to_string(),
        Element::P => "P".to_string(),
        Element::S => "S".to_string(),
        Element::F => "F".to_string(),
        Element::I => "I".to_string(),
        Element::As => "As".to_string(),
        Element::Br => "Br".to_string(),
        Element::Cl => "Cl".to_string(),
        Element::H2 => "[2]H".to_string(),
        Element::C13 => "[13]C".to_string(),
        Element::N15 => "[15]N".to_string(),
        Element::O17 => "[17]O".to_string(),
        Element::O18 => "[18]O".to_string(),
        Element::P32 => "[32]P".to_string(),
        Element::S33 => "[33]S".to_string(),
        Element::S34 => "[34]S".to_string(),
    });
}

// const map<Element, Element> heavy_to_regular = {{ELEMENT_H2, ELEMENT_H}, {ELEMENT_C13, ELEMENT_C}, {ELEMENT_N15, ELEMENT_N}, {ELEMENT_O17, ELEMENT_O}, {ELEMENT_O18, ELEMENT_O}, {ELEMENT_P32, ELEMENT_P}, {ELEMENT_S33, ELEMENT_S}, {ELEMENT_S34, ELEMENT_S}};
lazy_static! {
    static ref HEAVY_TO_REGULAR: Mutex<BTreeMap<Element, Element>> = Mutex::new(btreemap! {
        Element::H2 => Element::H,
        Element::C13 => Element::C,
        Element::N15 => Element::N,
        Element::O17 => Element::O,
        Element::O18 => Element::O,
        Element::P32 => Element::P,
        Element::S33 => Element::S,
        Element::S34 => Element::S,
    });
}

// const vector<Element> element_order = {ELEMENT_C, ELEMENT_H, ELEMENT_As, ELEMENT_Br, ELEMENT_Cl, ELEMENT_F, ELEMENT_I, ELEMENT_N, ELEMENT_O, ELEMENT_P, ELEMENT_S, ELEMENT_H2, ELEMENT_C13, ELEMENT_N15, ELEMENT_O17, ELEMENT_O18, ELEMENT_P32, ELEMENT_S33, ELEMENT_S34};
lazy_static! {
    static ref ELEMENT_ORDER: Mutex<Vec<Element>> = Mutex::new(vec![
        Element::C,
        Element::H,
        Element::As,
        Element::Br,
        Element::Cl,
        Element::F,
        Element::I,
        Element::N,
        Element::O,
        Element::P,
        Element::S,
        Element::H2,
        Element::C13,
        Element::N15,
        Element::O17,
        Element::O18,
        Element::P32,
        Element::S33,
        Element::S34,
    ]);
}

// const map<string, Element> heavy_element_table = {{"[2]H", ELEMENT_H2}, {"[13]C", ELEMENT_C13}, {"[15]N", ELEMENT_N15}, {"[17]O", ELEMENT_O17}, {"[18]O", ELEMENT_O18}, {"[32]P", ELEMENT_P32}, {"[33]S", ELEMENT_S33}, {"[34]S", ELEMENT_S34}};
lazy_static! {
    static ref HEAVY_ELEMENT_TABLE: Mutex<BTreeMap<String, Element>> = Mutex::new(btreemap! {
        "[2]H".to_string() => Element::H2,
        "[13]C".to_string() => Element::C13,
        "[15]N".to_string() => Element::N15,
        "[17]O".to_string() => Element::O17,
        "[18]O".to_string() => Element::O18,
        "[32]P".to_string() => Element::P32,
        "[33]S".to_string() => Element::S33,
        "[34]S".to_string() => Element::S34,
    });
}

pub fn compute_sum_formula(elements: &ElementTable) -> String {
    let mut ss = String::new();
    for e in ELEMENT_ORDER.lock().unwrap().iter() {
        if *elements.table.get(&e).unwrap() > 0 {
            ss.push_str(&ELEMENT_SHORTCUT.lock().unwrap().get(&e).unwrap());
        }
        if *elements.table.get(&e).unwrap() > 1 {
            ss.push_str(&elements.table.get(&e).unwrap().to_string());
        }
    }
    ss
}

/*
double goslin::get_mass(ElementTable *elements){
    double mass = 0;
    for (auto e : *elements) mass += element_masses.at(e.first) * e.second;
    return mass;
}

*/
pub fn compute_mass(elements: &ElementTable) -> f64 {
    let mut mass = 0.0;
    for (e, n) in elements.table.iter() {
        mass += *ELEMENT_MASSES.lock().unwrap().get(e).unwrap() * *n as f64;
    }
    mass
}
