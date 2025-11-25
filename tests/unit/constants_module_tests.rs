//! Tests for Constants native module
//!
//! Phase 11: Native stdlib modules

use graphoid::stdlib::{ConstantsModule, NativeModule};
use graphoid::values::ValueKind;

#[test]
fn test_constants_module_name() {
    let module = ConstantsModule;
    assert_eq!(module.name(), "constants");
}

#[test]
fn test_constants_module_alias() {
    let module = ConstantsModule;
    assert_eq!(module.alias(), Some("const"));
}

#[test]
fn test_constants_has_pi() {
    let module = ConstantsModule;
    let constants = module.constants();

    let pi = constants.get("pi").expect("Should have pi constant");
    match &pi.kind {
        ValueKind::Number(n) => {
            assert!((n - std::f64::consts::PI).abs() < 1e-10);
        }
        _ => panic!("pi should be a number"),
    }
}

#[test]
fn test_constants_has_e() {
    let module = ConstantsModule;
    let constants = module.constants();

    let e = constants.get("e").expect("Should have e constant");
    match &e.kind {
        ValueKind::Number(n) => {
            assert!((n - std::f64::consts::E).abs() < 1e-10);
        }
        _ => panic!("e should be a number"),
    }
}

#[test]
fn test_constants_has_tau() {
    let module = ConstantsModule;
    let constants = module.constants();

    let tau = constants.get("tau").expect("Should have tau constant");
    match &tau.kind {
        ValueKind::Number(n) => {
            assert!((n - std::f64::consts::TAU).abs() < 1e-10);
        }
        _ => panic!("tau should be a number"),
    }
}

#[test]
fn test_constants_has_phi() {
    let module = ConstantsModule;
    let constants = module.constants();

    let phi = constants.get("phi").expect("Should have phi (golden ratio) constant");
    match &phi.kind {
        ValueKind::Number(n) => {
            let expected_phi = 1.618033988749895;
            assert!((n - expected_phi).abs() < 1e-10);
        }
        _ => panic!("phi should be a number"),
    }
}

#[test]
fn test_constants_has_sqrt2() {
    let module = ConstantsModule;
    let constants = module.constants();

    let sqrt2 = constants.get("sqrt2").expect("Should have sqrt2 constant");
    match &sqrt2.kind {
        ValueKind::Number(n) => {
            assert!((n - std::f64::consts::SQRT_2).abs() < 1e-10);
        }
        _ => panic!("sqrt2 should be a number"),
    }
}

#[test]
fn test_constants_has_sqrt3() {
    let module = ConstantsModule;
    let constants = module.constants();

    let sqrt3 = constants.get("sqrt3").expect("Should have sqrt3 constant");
    match &sqrt3.kind {
        ValueKind::Number(n) => {
            let expected_sqrt3 = 3_f64.sqrt();
            assert!((n - expected_sqrt3).abs() < 1e-10);
        }
        _ => panic!("sqrt3 should be a number"),
    }
}

#[test]
fn test_constants_has_deg_to_rad() {
    let module = ConstantsModule;
    let constants = module.constants();

    let deg_to_rad = constants.get("deg_to_rad").expect("Should have deg_to_rad constant");
    match &deg_to_rad.kind {
        ValueKind::Number(n) => {
            let expected = std::f64::consts::PI / 180.0;
            assert!((n - expected).abs() < 1e-10);
        }
        _ => panic!("deg_to_rad should be a number"),
    }
}

#[test]
fn test_constants_has_rad_to_deg() {
    let module = ConstantsModule;
    let constants = module.constants();

    let rad_to_deg = constants.get("rad_to_deg").expect("Should have rad_to_deg constant");
    match &rad_to_deg.kind {
        ValueKind::Number(n) => {
            let expected = 180.0 / std::f64::consts::PI;
            assert!((n - expected).abs() < 1e-10);
        }
        _ => panic!("rad_to_deg should be a number"),
    }
}

#[test]
fn test_constants_has_speed_of_light() {
    let module = ConstantsModule;
    let constants = module.constants();

    let c = constants.get("c").expect("Should have c (speed of light) constant");
    match &c.kind {
        ValueKind::Number(n) => {
            assert_eq!(*n, 299792458.0);
        }
        _ => panic!("c should be a number"),
    }
}

#[test]
fn test_constants_has_gravitational_constant() {
    let module = ConstantsModule;
    let constants = module.constants();

    let g = constants.get("G").expect("Should have G (gravitational constant)");
    match &g.kind {
        ValueKind::Number(n) => {
            let expected = 6.67430e-11;
            assert!((n - expected).abs() < 1e-16);
        }
        _ => panic!("G should be a number"),
    }
}

#[test]
fn test_constants_has_planck_constant() {
    let module = ConstantsModule;
    let constants = module.constants();

    let h = constants.get("h").expect("Should have h (Planck constant)");
    match &h.kind {
        ValueKind::Number(n) => {
            let expected = 6.62607015e-34;
            assert!((n - expected).abs() < 1e-40);
        }
        _ => panic!("h should be a number"),
    }
}

#[test]
fn test_constants_no_functions() {
    let module = ConstantsModule;
    let functions = module.functions();

    assert!(functions.is_empty(), "Constants module should have no functions");
}

#[test]
fn test_constants_count() {
    let module = ConstantsModule;
    let constants = module.constants();

    assert!(constants.len() >= 12, "Should have at least 12 constants");
}
