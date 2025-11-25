//! Random Module - Random number generation, UUIDs, and secure tokens

use super::{NativeFunction, NativeModule};
use crate::error::{GraphoidError, Result};
use crate::values::{Value, ValueKind, List};
use std::collections::HashMap;
use std::cell::RefCell;
use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng, ThreadRng};
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Normal, Exp};

thread_local! {
    static SEEDED_RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

pub struct RandomModule;

impl RandomModule {
    pub fn new() -> Self {
        RandomModule
    }

    fn get_thread_rng() -> ThreadRng {
        rand::thread_rng()
    }

    fn seed_rng(seed: u64) {
        SEEDED_RNG.with(|rng| {
            *rng.borrow_mut() = Some(StdRng::seed_from_u64(seed));
        });
    }

    fn get_seeded_rng() -> Result<StdRng> {
        SEEDED_RNG.with(|rng| {
            rng.borrow().clone().ok_or_else(|| GraphoidError::runtime(
                "Deterministic RNG not seeded. Call rand.seed(value) first.".to_string()
            ))
        })
    }
}

impl NativeModule for RandomModule {
    fn name(&self) -> &str {
        "random"
    }

    fn alias(&self) -> Option<&str> {
        Some("rand")
    }

    fn functions(&self) -> HashMap<String, NativeFunction> {
        let mut functions = HashMap::new();
        functions.insert("random".to_string(), random as NativeFunction);
        functions.insert("randint".to_string(), randint as NativeFunction);
        functions.insert("uniform".to_string(), uniform as NativeFunction);
        functions.insert("choice".to_string(), choice as NativeFunction);
        functions.insert("shuffle".to_string(), shuffle as NativeFunction);
        functions.insert("sample".to_string(), sample as NativeFunction);
        functions.insert("normal".to_string(), normal as NativeFunction);
        functions.insert("exponential".to_string(), exponential as NativeFunction);
        functions.insert("seed".to_string(), seed as NativeFunction);
        functions.insert("det_random".to_string(), det_random as NativeFunction);
        functions.insert("det_randint".to_string(), det_randint as NativeFunction);
        functions.insert("uuid4".to_string(), uuid4 as NativeFunction);
        functions.insert("token".to_string(), token as NativeFunction);
        functions.insert("token_urlsafe".to_string(), token_urlsafe as NativeFunction);
        functions
    }
}

fn random(_args: &[Value]) -> Result<Value> {
    let mut rng = RandomModule::get_thread_rng();
    Ok(Value::number(rng.gen()))
}

fn randint(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("randint() requires 2 arguments: min and max".to_string()));
    }

    let min = match &args[0].kind {
        ValueKind::Number(n) => *n as i64,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let max = match &args[1].kind {
        ValueKind::Number(n) => *n as i64,
        _ => return Err(GraphoidError::type_error("number", args[1].type_name())),
    };

    if min > max {
        return Err(GraphoidError::runtime(format!("randint() min ({}) must be <= max ({})", min, max)));
    }

    let mut rng = RandomModule::get_thread_rng();
    Ok(Value::number(rng.gen_range(min..=max) as f64))
}

fn uniform(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("uniform() requires 2 arguments: min and max".to_string()));
    }

    let min = match &args[0].kind {
        ValueKind::Number(n) => *n,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let max = match &args[1].kind {
        ValueKind::Number(n) => *n,
        _ => return Err(GraphoidError::type_error("number", args[1].type_name())),
    };

    if min >= max {
        return Err(GraphoidError::runtime(format!("uniform() min ({}) must be < max ({})", min, max)));
    }

    let mut rng = RandomModule::get_thread_rng();
    Ok(Value::number(Uniform::new(min, max).sample(&mut rng)))
}

fn choice(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("choice() requires a list argument".to_string()));
    }

    match &args[0].kind {
        ValueKind::List(list) => {
            if list.is_empty() {
                return Err(GraphoidError::runtime("choice() cannot choose from empty list".to_string()));
            }
            let mut rng = RandomModule::get_thread_rng();
            let index = rng.gen_range(0..list.len());
            list.get(index).cloned().ok_or_else(|| GraphoidError::runtime("Invalid index".to_string()))
        }
        _ => Err(GraphoidError::type_error("list", args[0].type_name())),
    }
}

fn shuffle(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("shuffle() requires a list argument".to_string()));
    }

    match &args[0].kind {
        ValueKind::List(list) => {
            use rand::seq::SliceRandom;
            let mut rng = RandomModule::get_thread_rng();
            let mut vec = list.to_vec();
            vec.shuffle(&mut rng);
            Ok(Value::list(List::from_vec(vec)))
        }
        _ => Err(GraphoidError::type_error("list", args[0].type_name())),
    }
}

fn sample(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("sample() requires 2 arguments: list and count".to_string()));
    }

    let k = match &args[1].kind {
        ValueKind::Number(n) => *n as usize,
        _ => return Err(GraphoidError::type_error("number", args[1].type_name())),
    };

    match &args[0].kind {
        ValueKind::List(list) => {
            use rand::seq::SliceRandom;
            if k > list.len() {
                return Err(GraphoidError::runtime(format!("sample() count ({}) exceeds list length ({})", k, list.len())));
            }
            let mut rng = RandomModule::get_thread_rng();
            let vec = list.to_vec();
            let sampled: Vec<Value> = vec.choose_multiple(&mut rng, k).cloned().collect();
            Ok(Value::list(List::from_vec(sampled)))
        }
        _ => Err(GraphoidError::type_error("list", args[0].type_name())),
    }
}

fn normal(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("normal() requires 2 arguments: mean and std_dev".to_string()));
    }

    let mean = match &args[0].kind {
        ValueKind::Number(n) => *n,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let std_dev = match &args[1].kind {
        ValueKind::Number(n) => *n,
        _ => return Err(GraphoidError::type_error("number", args[1].type_name())),
    };

    if std_dev <= 0.0 {
        return Err(GraphoidError::runtime("normal() std_dev must be positive".to_string()));
    }

    let mut rng = RandomModule::get_thread_rng();
    let dist = Normal::new(mean, std_dev)
        .map_err(|e| GraphoidError::runtime(format!("Failed to create normal distribution: {}", e)))?;
    Ok(Value::number(dist.sample(&mut rng)))
}

fn exponential(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("exponential() requires lambda argument".to_string()));
    }

    let lambda = match &args[0].kind {
        ValueKind::Number(n) => *n,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    if lambda <= 0.0 {
        return Err(GraphoidError::runtime("exponential() lambda must be positive".to_string()));
    }

    let mut rng = RandomModule::get_thread_rng();
    let dist = Exp::new(lambda)
        .map_err(|e| GraphoidError::runtime(format!("Failed to create exponential distribution: {}", e)))?;
    Ok(Value::number(dist.sample(&mut rng)))
}

fn seed(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("seed() requires a number argument".to_string()));
    }

    let seed_value = match &args[0].kind {
        ValueKind::Number(n) => *n as u64,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    RandomModule::seed_rng(seed_value);
    Ok(Value::none())
}

fn det_random(_args: &[Value]) -> Result<Value> {
    let mut rng = RandomModule::get_seeded_rng()?;
    let value: f64 = rng.gen();
    SEEDED_RNG.with(|rng_cell| *rng_cell.borrow_mut() = Some(rng));
    Ok(Value::number(value))
}

fn det_randint(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(GraphoidError::runtime("det_randint() requires 2 arguments: min and max".to_string()));
    }

    let min = match &args[0].kind {
        ValueKind::Number(n) => *n as i64,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let max = match &args[1].kind {
        ValueKind::Number(n) => *n as i64,
        _ => return Err(GraphoidError::type_error("number", args[1].type_name())),
    };

    if min > max {
        return Err(GraphoidError::runtime(format!("det_randint() min ({}) must be <= max ({})", min, max)));
    }

    let mut rng = RandomModule::get_seeded_rng()?;
    let value = rng.gen_range(min..=max);
    SEEDED_RNG.with(|rng_cell| *rng_cell.borrow_mut() = Some(rng));
    Ok(Value::number(value as f64))
}

fn uuid4(_args: &[Value]) -> Result<Value> {
    Ok(Value::string(uuid::Uuid::new_v4().to_string()))
}

fn token(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("token() requires length argument (in bytes)".to_string()));
    }

    let length = match &args[0].kind {
        ValueKind::Number(n) => *n as usize,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let mut rng = RandomModule::get_thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    let hex = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    Ok(Value::string(hex))
}

fn token_urlsafe(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(GraphoidError::runtime("token_urlsafe() requires length argument (in bytes)".to_string()));
    }

    let length = match &args[0].kind {
        ValueKind::Number(n) => *n as usize,
        _ => return Err(GraphoidError::type_error("number", args[0].type_name())),
    };

    let mut rng = RandomModule::get_thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let token: String = bytes.iter().map(|&b| CHARSET[(b % CHARSET.len() as u8) as usize] as char).collect();
    Ok(Value::string(token))
}
