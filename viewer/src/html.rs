use std::collections::HashMap;
use std::sync::OnceLock;
use minijinja::{context, Environment, Error};
use l6t::symbolic::value::Value;
use crate::file::Patch;

fn env() -> &'static Environment<'static> {
    static ENV: OnceLock<Environment> = OnceLock::new();
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.set_debug(true);
        env.add_template("empty.html", include_str!("../data/empty.html")).unwrap();
        env.add_template("patch.html", include_str!("../data/patch.html")).unwrap();
        env.add_template("patch.css", include_str!("../data/patch.css")).unwrap();
        env.add_filter("hex", hex);
        env
    })
}

fn hex(value: u32) -> String {
    format!("{:#04x}", value)
}


pub fn generate_html(patch: &Patch) -> String {
    let env = env();
    let t = env.get_template("patch.html").unwrap();

    // render all template values to a values hash map
    let mut values = HashMap::new();
    let mut raw = HashMap::new();
    for g in &patch.values {
        for (n, v) in &g.values {
            values.insert(n.clone(), v.to_string());

            let extra = match v.get_simple() {
                Value::Int(v) => format!(" ({:#04x})", v),
                _ => String::new()
            };
            raw.insert(n.clone(), format!("{}: {}{}", v.get_simple_type(), v.get_simple(), extra));
        }
    }

    let context = context!(
        patch => patch.patch,
        groups => patch.values,
        values => values,
        raw => raw,
        errors => patch.errors,
    );

    match t.render(context) {
        Ok(v) => { v }
        Err(e) => {
            println!("ERROR: {e}");
            "".into()
        }
    }
}
pub fn generate_empty() -> String {
    let env = env();
    let t = env.get_template("empty.html").unwrap();

    t.render(context!()).unwrap()
}
