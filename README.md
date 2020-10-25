# ssb-keys-mnemonic-neon

> A drop-in replacement of `ssb-keys-mnemonic`, implemented in Rust and delivered as a native module in Node.js

```
npm install ssb-keys-mnemonic-neon
```

Read more about [SSB Neon here](https://github.com/ssb-neon/ssb-neon).

## Usage

There are two ways you can use this npm package.

### Option 1: (easiest) automatically replace

In your **package.json**, assuming you already have `ssb-keys-mnemonic`, you can replace its *implementation* by pointing to the GitHub repo for `ssb-keys-mnemonic-neon`:

```diff
   // ...
   "dependencies": {
     "ssb-ebt": "^5.6.7",
     "ssb-friends": "^4.1.4",
     "ssb-invite": "^2.1.3",
-    "ssb-keys-mnemonic": "1.0.0",
+    "ssb-keys-mnemonic": "staltz/ssb-keys-mnemonic-neon#replace-1.0.0",
     "ssb-lan": "^0.2.0",
     "ssb-logging": "^1.0.0",
     "ssb-markdown": "^6.0.4",
   }
   // ...
```

This is the easiest method because you only need to change `package.json`, your code can still `require('ssb-keys-mnemonic')` and under the hood it will load `ssb-keys-mnemonic-neon`.

Note that you **cannot specify version ranges**. If you previously had `"ssb-keys-mnemonic": "1.0.x"`, you will have to specify an exact version when you write `"staltz/ssb-keys-mnemonic-neon#replace-1.0.0"`, you cannot write `"staltz/ssb-keys-mnemonic-neon#replace-1.0.x"`.

### Option 2: manually replace

This method gives you more control over the usage of `ssb-keys-mnemonic-neon`, as well as allows you to specify version ranges. Just *remove ssb-keys-mnemonic* from your package.json, and *add ssb-keys-mnemonic-neon*:

```diff
   // ...
   "dependencies": {
     "ssb-ebt": "^5.6.7",
     "ssb-friends": "^4.1.4",
     "ssb-invite": "^2.1.3",
-    "ssb-keys-mnemonic": "1.0.0",
+    "ssb-keys-mnemonic-neon": ">=1.0.0-1",
     "ssb-lan": "^0.2.0",
     "ssb-logging": "^1.0.0",
     "ssb-markdown": "^6.0.4",
   }
   // ...
```

**Then**, you also have to replace usages of ssb-keys manually in JavaScript source files:

```diff
-var ssbKeys = require('ssb-keys-mnemonic')
+var ssbKeys = require('ssb-keys-mnemonic-neon')

 // ...
```

## Versioning and support

`ssb-keys-mnemonic-neon@X.Y.Z-num` is compatible with `ssb-keys-mnemonic@X.Y.Z`.

Versions of ssb-keys-mnemonic that are mirrored by ssb-keys-mnemonic-neon currently include (and which platforms are prebuilt for):

<details>
<summary>1.0.0 (click here to see which platforms have prebuilt binaries)</summary>

As of `ssb-keys-mnemonic-neon@1.0.0-1`

- macOS (darwin-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x
- Linux (linux-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x
- Windows (win32-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x

</details>


<details>
<summary>0.3.1 (click here to see which platforms have prebuilt binaries)</summary>

As of `ssb-keys-mnemonic-neon@0.3.1-2`

- macOS (darwin-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x
- Linux (linux-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x
- Windows (win32-x64)
  - Node 10.x
  - Node 12.x
  - Node 14.x
  - Electron 7.x
  - Electron 8.x
  - Electron 9.x
  - Electron 10.x

</details>

## License

AGPL-3.0
