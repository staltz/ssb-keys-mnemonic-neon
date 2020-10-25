use super::utils::{make_keys_obj, HandleExt, OptionExt};
use bip39::{Language, Mnemonic};
use neon::prelude::*;
use ssb_crypto::Keypair;

pub fn neon_words_to_keys(mut cx: FunctionContext) -> JsResult<JsObject> {
  let argc = cx.len();
  if argc < 1 {
    return cx.throw_error("wordsToKeys requires an argument: the words string");
  }

  let words = cx
    .argument::<JsValue>(0)?
    .try_downcast::<JsString>()
    .or_throw(&mut cx, "wordsToKeys requires the argument to be a string")?
    .value();

  let mut words_vec: Vec<&str> = words.as_str().trim().split(char::is_whitespace).collect();
  if words_vec.len() < 24 || words_vec.len() > 48 {
    return cx.throw_error("there should be 24 words");
  }
  words_vec.truncate(24);

  let phrase = String::from(words_vec.join(" "));

  let mnemonic = Mnemonic::from_phrase(phrase, Language::English);
  if mnemonic.is_err() {
    return cx.throw_error("invalid words");
  }
  let mnemonic = mnemonic.unwrap();

  let seed_bytes = mnemonic.entropy();

  let keypair = Keypair::from_seed(&seed_bytes).or_throw(&mut cx, "invalid words")?;

  make_keys_obj(&mut cx, &keypair)
}
