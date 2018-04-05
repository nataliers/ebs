#[macro_use]
extern crate neon;

extern crate bytebuffer;

use neon::vm::{Call, JsResult, Throw, Lock};
use neon::scope::RootScope;
use neon::js::{JsFunction, JsValue, JsArray, JsNull, JsBoolean, JsNumber, JsInteger, JsString,
               Object, Value, Variant, JsUndefined, JsObject};
use neon::js::binary::JsBuffer;
use neon::mem::{Handle, Managed};
use bytebuffer::ByteBuffer;

const T_NULL: u8 = 1;
const T_FALSE: u8 = T_NULL + 1;
const T_TRUE: u8 = T_NULL + 2;
const T_ESTRING: u8 = T_NULL + 3;
const T_EARRAY: u8 = T_NULL + 4;
const T_EOBJECT: u8 = T_NULL + 5;
const T_INTEGER: u8 = T_NULL + 6;
const T_NUMBER: u8 = T_NULL + 7;
const T_STRING: u8 = T_NULL + 8;
const T_ARRAY: u8 = T_NULL + 9;
const T_OBJECT: u8 = T_NULL + 10;

fn serialize(call: Call) -> JsResult<JsBuffer> {
  let scope = call.scope;
  let value = call.arguments.get(scope, 0).unwrap();
  let mut buffer = ByteBuffer::new();
  s_value(&mut buffer, value, scope);
  let bytes = buffer.to_bytes();
  let mut js_buffer = JsBuffer::new(scope, bytes.len() as u32).unwrap();
  js_buffer.grab(|mut contents| contents.as_mut_slice().copy_from_slice(&bytes));
  Ok(js_buffer)
}

fn s_value(buffer: &mut ByteBuffer, value: Handle<JsValue>, scope: &mut RootScope) {
  match value.variant() {
    Variant::Null(_) => {
      s_null(buffer);
    }
    Variant::Boolean(value) => {
      s_boolean(buffer, value);
    }
    Variant::Integer(value) => {
      s_integer(buffer, value);
    }
    Variant::Number(value) => {
      s_number(buffer, value);
    }
    Variant::String(value) => {
      s_string(buffer, value);
    }
    Variant::Array(value) => {
      s_array(buffer, value, scope);
    }
    Variant::Object(value) => {
      s_object(buffer, value, scope);
    }
    _ => {
      // Ignore unknown type
    }
  }
}

fn s_null(buffer: &mut ByteBuffer) {
  buffer.write_u8(T_NULL);
}

fn s_boolean(buffer: &mut ByteBuffer, value: Handle<JsBoolean>) {
  if value.value() {
    buffer.write_u8(T_TRUE);
  } else {
    buffer.write_u8(T_FALSE);
  }
}

fn s_integer(buffer: &mut ByteBuffer, value: Handle<JsInteger>) {
  buffer.write_u8(T_INTEGER);
  buffer.write_i64(value.value());
}

fn s_number(buffer: &mut ByteBuffer, value: Handle<JsNumber>) {
  buffer.write_u8(T_NUMBER);
  buffer.write_f64(value.value());
}

fn s_string(buffer: &mut ByteBuffer, value: Handle<JsString>) {
  buffer.write_u8(T_STRING);
  buffer.write_string(&value.value());
}

fn s_array(buffer: &mut ByteBuffer, value: Handle<JsArray>, scope: &mut RootScope) {
  let len = value.len();
  if len == 0 {
    buffer.write_u8(T_EARRAY);
  } else {
    buffer.write_u8(T_ARRAY);
    buffer.write_u32(len);
    for idx in 0..len {
      s_value(buffer, value.get(scope, idx).unwrap(), scope);
    }
  }
}

fn s_object(buffer: &mut ByteBuffer, value: Handle<JsObject>, scope: &mut RootScope) {
  let keys = value.get_own_property_names(scope).unwrap();
  let len = keys.len();
  if len == 0 {
    buffer.write_u8(T_EOBJECT);
  } else {
    buffer.write_u8(T_OBJECT);
    buffer.write_u32(len);
    for idx in 0..len {
      let key = keys.get(scope, idx).unwrap().downcast::<JsString>().unwrap().value();
      println!("{:?}", key);
      buffer.write_string(&key);
      s_value(buffer, value.get(scope, key.as_str()).unwrap(), scope);
    }
  }
}

register_module!(m, {
  m.export("serialize", serialize)
});
