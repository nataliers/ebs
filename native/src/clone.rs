use neon::vm::{Call, JsResult, Throw, Lock};
use neon::scope::RootScope;
use neon::js::{JsFunction, JsValue, JsArray, JsNull, JsBoolean, JsNumber, JsInteger, JsString,
               Object, Value, Variant, JsUndefined, JsObject};
use neon::js::binary::JsArrayBuffer;
use neon::mem::{Handle, Managed};
use bytebuffer::ByteBuffer;

use {T_OBJECT, T_ARRAY, T_NULL, T_FALSE, T_TRUE, T_EARRAY, T_EOBJECT, T_INTEGER, T_NUMBER, T_STRING};

pub fn clone(call: Call) -> JsResult<JsValue> {
  let scope = call.scope;
  let value = call.arguments.get(scope, 0).unwrap();
  Ok(c_value(value, scope))
}

fn c_value<'a>(value: Handle<'a, JsValue>, scope: &mut RootScope<'a>) -> Handle<'a, JsValue> {
  match value.variant() {
    Variant::Null(_) => {
      value
    }
    Variant::Boolean(_) => {
      value
    }
    Variant::Integer(_) => {
      value
    }
    Variant::Number(_) => {
      value
    }
    Variant::String(_) => {
      value
    }
    Variant::Array(value) => {
      c_array(value, scope)
    }
    Variant::Object(value) => {
      c_object(value, scope)
    }
    _ => {
      value
    }
  }
}

fn c_array<'a>(value: Handle<'a, JsArray>, scope: &mut RootScope<'a>) -> Handle<'a, JsValue> {
  let len = value.len();
  let array = JsArray::new(scope, len);
  if len > 0 {
    for idx in 0..len {
      array.set(idx, c_value(value.get(scope, idx).unwrap(), scope));
    }
  }
  array.upcast::<JsValue>()
}

fn c_object<'a>(value: Handle<'a, JsObject>, scope: &mut RootScope<'a>) -> Handle<'a, JsValue> {
  let keys = value.get_own_property_names(scope).unwrap();
  let len = keys.len();
  let object = JsObject::new(scope);
  if len > 0 {
    for idx in 0..len {
      let key = keys.get(scope, idx).unwrap();
      object.set(key, c_value(value.get(scope, key).unwrap(), scope));
    }
  }
  object.upcast::<JsValue>()
}
