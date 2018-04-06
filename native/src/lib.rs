#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsValue;

extern crate bytebuffer;

mod types;

use types::*;

mod serialize;

use serialize::serialize;

mod deserialize;

use deserialize::deserialize;

pub fn echo(call: Call) -> JsResult<JsValue> {
  let scope = call.scope;
  let value = call.arguments.get(scope, 0).unwrap();
  Ok(value)
}

register_module!(m, {
  m.export("serialize", serialize);
  m.export("deserialize", deserialize);
  m.export("echo", echo);
  Ok(())
});
