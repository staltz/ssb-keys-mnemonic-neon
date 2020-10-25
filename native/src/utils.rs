use neon::prelude::*;
use ssb_crypto::Keypair;

pub fn make_keys_obj<'a>(cx: &mut impl Context<'a>, kp: &Keypair) -> JsResult<'a, JsObject> {
  let keys_obj = JsObject::new(cx);
  let curve_val = cx.string("ed25519");
  let id_val = cx.string(kp.public.as_base64().wrap('@', ".ed25519"));
  let private_val = cx.string(kp.as_base64().with_suffix(".ed25519"));
  let public_val = cx.string(kp.public.as_base64().with_suffix(".ed25519"));
  keys_obj.set(cx, "curve", curve_val)?;
  keys_obj.set(cx, "id", id_val)?;
  keys_obj.set(cx, "private", private_val)?;
  keys_obj.set(cx, "public", public_val)?;
  Ok(keys_obj)
}

pub trait StringExt {
  fn with_suffix(self, s: &str) -> Self;
  fn wrap(self, prefix: char, suffix: &str) -> Self;
}

impl StringExt for String {
  fn with_suffix(mut self, s: &str) -> Self {
    self.push_str(s);
    self
  }
  fn wrap(mut self, prefix: char, suffix: &str) -> Self {
    self.insert(0, prefix);
    self.push_str(suffix);
    self
  }
}

pub trait OptionExt<T> {
  fn or_throw<'a, S: AsRef<str>>(
    self,
    cx: &mut impl Context<'a>,
    msg: S,
  ) -> Result<T, neon::result::Throw>;
}

impl<T> OptionExt<T> for Option<T> {
  fn or_throw<'a, S: AsRef<str>>(
    self,
    cx: &mut impl Context<'a>,
    msg: S,
  ) -> Result<T, neon::result::Throw> {
    // Result<T, _>::unwrap_err and expect_err require T: Debug, which JsArray doesn't impl
    self.ok_or_else(|| cx.throw_error::<_, T>(msg).err().unwrap())
  }
}

// `if let Ok(s) = v.downcast::<JsString>() { ... }`
// can be used with zero cost (aside from the type tag check)
// when this PR is merged: https://github.com/neon-bindings/neon/pull/606
//
// In the meantime, we'll use this:
pub trait HandleExt<'a> {
  fn try_downcast<U: Value>(&self) -> Option<Handle<'a, U>>;
}
impl<'a, T: Value> HandleExt<'a> for Handle<'a, T> {
  fn try_downcast<U: Value>(&self) -> Option<Handle<'a, U>> {
    if self.is_a::<U>() {
      Some(self.downcast::<U>().unwrap())
    } else {
      None
    }
  }
}

// Like `cx.argument::<T>(index)?` but with a custom error msg
pub trait ContextExt<'a> {
  fn arg_as<T: Value>(
    &mut self,
    index: i32,
    msg: &str,
  ) -> Result<Handle<'a, T>, neon::result::Throw>;
}

impl<'a> ContextExt<'a> for FunctionContext<'a> {
  fn arg_as<T: Value>(
    &mut self,
    index: i32,
    msg: &str,
  ) -> Result<Handle<'a, T>, neon::result::Throw> {
    let v = self.argument::<JsValue>(index)?;
    v.try_downcast::<T>().or_throw(self, msg)
  }
}
