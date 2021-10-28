use rustler::types::map;
use rustler::{Atom, Env, Error, Term, NifResult};
use rustler::Encoder;

mod atoms {
    rustler::atoms! {
        lauren,
        error,
        a, b, c,
    }
}

#[rustler::nif]
fn make_int() -> i64 {
    42
}

#[rustler::nif]
fn make_float() -> f64 {
    3.14159260123456789
}

#[rustler::nif]
fn make_char() -> u8 {
    'a' as u8
}

#[rustler::nif]
fn make_charlist() -> Vec<u8> {
    "lauren".bytes().map(|c| c as u8).collect()
}

#[rustler::nif]
fn make_string() -> String {
    String::from("Lauren")
}

#[rustler::nif]
fn make_tuple<'a>(env: Env<'a>) -> Term<'a> {
    (1,2,3).encode(env)
}

#[rustler::nif]
fn make_atom() -> Atom {
    atoms::lauren()
}

#[rustler::nif]
fn make_list() -> Vec<u8> {
    (0..5).collect()
}

#[rustler::nif]
fn make_map<'a>(env: Env<'a>) -> Term<'a> {
    let mut map = map::map_new(env);
    map = map.map_put(atoms::a().to_term(env), 1.encode(env)).unwrap();
    map = map.map_put(atoms::b().to_term(env), 2.encode(env)).unwrap();
    map = map.map_put(atoms::c().to_term(env), 3.encode(env)).unwrap();

    map
}

#[rustler::nif]
fn add1<'a>(env: Env<'a>, term: Term) -> NifResult<Term<'a>> {
    if let Ok(i) = term.decode::<i64>() {
        Ok((i + 1).encode(env))
    } else if let Ok(f) = term.decode::<f64>() {
        Ok((f + 1.0).encode(env))
    } else {
        Err(Error::BadArg)
    }
}

#[rustler::nif]
fn exclaim(s: String) -> String {
    s.clone() + "!"
}

rustler::init!(
    "Elixir.Basics",
    [
        make_int,
        make_float,
        make_char,
        make_charlist,
        make_string,
        make_tuple,
        make_atom,
        make_list,
        make_map,
        add1,
        exclaim,
    ]
);
