use super::utils::{HandleExt, OptionExt};
use bip39::{Language, Mnemonic};
use neon::prelude::*;
use ssb_crypto::{Keypair, SecretKey};

pub fn neon_keys_to_words(mut cx: FunctionContext) -> JsResult<JsString> {
  let argc = cx.len();
  if argc < 1 {
    return cx.throw_error("keysToWords requires an argument: the keys object");
  }

  let input = cx.argument::<JsValue>(0)?.try_downcast::<JsObject>();

  let curve_str = input
    .and_then(|obj| obj.get(&mut cx, "curve").ok())
    .and_then(|field| field.try_downcast::<JsString>())
    .or_throw(
      &mut cx,
      "keysToWords requires argument to have field 'curve'",
    )?
    .value();

  if curve_str != "ed25519" {
    return cx.throw_error("only ed25519 is supported");
  }

  input
    .and_then(|obj| obj.get(&mut cx, "public").ok())
    .and_then(|field| field.try_downcast::<JsString>())
    .or_throw(&mut cx, "keys object is missing .public field")?;

  let private_str = input
    .and_then(|obj| obj.get(&mut cx, "private").ok())
    .and_then(|field| field.try_downcast::<JsString>())
    .or_throw(&mut cx, "keys object is missing .private field")?
    .value();

  let keypair =
    Keypair::from_base64(&private_str).or_throw(&mut cx, "cannot decode private key bytes")?;

  let SecretKey(seed) = keypair.secret;

  let mnemonic = Mnemonic::from_entropy(&seed, Language::English);

  if mnemonic.is_err() {
    return cx.throw_error("keysToWords failed to convert to a 24-word phrase");
  }

  let phrase = mnemonic.unwrap();

  Ok(cx.string(phrase))
}
