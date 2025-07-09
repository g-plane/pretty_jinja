# Pretty Jinja

Pretty Jinja is a formatter that formats Jinja expressions and statements.

## Usage

You should use this as a dprint plugin together with [markup_fmt](https://github.com/g-plane/markup_fmt) in dprint.

Run the commands below to add plugins:

```bash
dprint config add g-plane/markup_fmt
dprint config add g-plane/pretty_jinja
```

After adding the dprint plugins, update your `dprint.json` and add configuration:

```jsonc
{
  // ...
  "plugins": [
    // ... other plugins URL
    "https://plugins.dprint.dev/g-plane/pretty_jinja-v0.1.0.wasm"
  ],
  "jinja": { // <-- the key name here is "jinja", not "pretty_jinja"
    // config comes here
  }
}
```

## License

MIT License

Copyright (c) 2025-present Pig Fang
