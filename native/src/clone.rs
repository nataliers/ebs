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

fn c_value<'a>(value: Handle<JsValue>, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  match value.variant() {
    Variant::Null(_) => {
      JsNull::new().upcast::<JsValue>()
    }
    Variant::Boolean(value) => {
      JsBoolean::new(scope, value.value()).upcast::<JsValue>()
    }
    Variant::Integer(value) => {
      JsInteger::new(scope, value.value() as i32).upcast::<JsValue>()
    }
    Variant::Number(value) => {
      JsNumber::new(scope, value.value()).upcast::<JsValue>()
    }
    Variant::String(value) => {
      JsString::new(scope, value.value().as_str()).unwrap().upcast::<JsValue>()
    }
    Variant::Array(value) => {
      c_array(value, scope)
    }
    Variant::Object(value) => {
      c_object(value, scope)
    }
    _ => {
      JsNull::new().upcast::<JsValue>()
    }
  }
}

fn c_array<'a>(value: Handle<JsArray>, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  let len = value.len();
  let array = JsArray::new(scope, len);
  if len > 0 {
    for idx in 0..len {
      array.set(idx, c_value(value.get(scope, idx).unwrap(), scope));
    }
  }
  array.upcast::<JsValue>()
}

fn c_object<'a>(value: Handle<JsObject>, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
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
