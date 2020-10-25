mod k2w;
mod utils;
mod w2k;

use self::k2w::neon_keys_to_words;
use self::w2k::neon_words_to_keys;
use neon::prelude::*;

register_module!(mut cx, {
  cx.export_function("keysToWords", neon_keys_to_words)?;
  cx.export_function("wordsToKeys", neon_words_to_keys)?;
  Ok(())
});
