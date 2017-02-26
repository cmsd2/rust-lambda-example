extern crate rust_apex;
extern crate serde_json;
#[macro_use]
extern crate quick_error;

use std::collections::BTreeMap;
use std::fmt;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        FmtError(e: fmt::Error) {
            from()
            description("format error")
            display("Format error: {}", e)
            cause(e)
        }

        JsonError(e: serde_json::Error) {
            from()
            description("json error")
            display("Json error: {}", e)
            cause(e)
        }
    }
}

fn main() {
    rust_apex::run::<_, _, Error, _>(|input: serde_json::Value, c: rust_apex::Context| {
        let mut bt = BTreeMap::new();
	bt.insert("c", serde_json::to_value(&c)?);
        bt.insert("i", input);
        Ok(bt)
    });
}
