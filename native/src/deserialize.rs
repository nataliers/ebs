use neon::vm::{Call, JsResult, Throw, Lock};
use neon::scope::RootScope;
use neon::js::{JsFunction, JsValue, JsArray, JsNull, JsBoolean, JsNumber, JsInteger, JsString,
               Object, Value, Variant, JsUndefined, JsObject};
use neon::js::binary::JsArrayBuffer;
use neon::mem::{Handle, Managed};
use bytebuffer::ByteBuffer;

use {T_OBJECT, T_ARRAY, T_NULL, T_FALSE, T_TRUE, T_EARRAY, T_EOBJECT, T_INTEGER, T_NUMBER, T_STRING};

pub fn deserialize(call: Call) -> JsResult<JsValue> {
  let scope = call.scope;
  let mut js_buffer = call.arguments.get(scope, 0).unwrap().downcast::<JsArrayBuffer>().unwrap();
  let mut buffer = ByteBuffer::from_bytes(js_buffer.grab(|mut contents| contents.as_slice()));
  Ok(ds_value(&mut buffer, scope))
}

fn ds_value<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  match buffer.read_u8() {
    T_NULL => {
      JsNull::new().upcast::<JsValue>()
    }
    T_TRUE => {
      JsBoolean::new(scope, true).upcast::<JsValue>()
    }
    T_FALSE => {
      JsBoolean::new(scope, false).upcast::<JsValue>()
    }
    T_EARRAY => {
      JsArray::new(scope, 0).upcast::<JsValue>()
    }
    T_EOBJECT => {
      JsObject::new(scope).upcast::<JsValue>()
    }
    T_INTEGER => {
      ds_integer(buffer, scope)
    }
    T_NUMBER => {
      ds_number(buffer, scope)
    }
    T_STRING => {
      ds_string(buffer, scope)
    }
    T_ARRAY => {
      ds_array(buffer, scope)
    }
    T_OBJECT => {
      ds_object(buffer, scope)
    }
    _ => JsUndefined::new().upcast::<JsValue>()
  }
}

fn ds_integer<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  JsInteger::new(scope, buffer.read_i32()).upcast::<JsValue>()
}

fn ds_number<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  JsNumber::new(scope, buffer.read_f64()).upcast::<JsValue>()
}

fn ds_string<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  JsString::new(scope, buffer.read_string().as_str()).unwrap().upcast::<JsValue>()
}

fn ds_array<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  let len = buffer.read_u32();
  let array = JsArray::new(scope, len);
  if len > 0 {
    for idx in 0..len {
      array.set(idx, ds_value(buffer, scope));
    }
  }
  array.upcast::<JsValue>()
}

fn ds_object<'a>(buffer: &mut ByteBuffer, scope: &'a mut RootScope) -> Handle<'a, JsValue> {
  let len = buffer.read_u32();
  let object = JsObject::new(scope);
  if len > 0 {
    for idx in 0..len {
      let key = buffer.read_string();
      object.set(key.as_str(), ds_value(buffer, scope));
    }
  }
  object.upcast::<JsValue>()
}
