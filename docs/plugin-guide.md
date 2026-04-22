# WASM Plugin Development Guide

`patch-ts` supports WebAssembly plugins for custom repair logic. Plugins implement the `repair` interface defined in [WIT](https://component-model.bytecodealliance.org/).

## Interface Definition

```wit
package patch-ts:plugin;

interface repair {
    record span {
        start-byte: u32,
        end-byte: u32,
    }

    variant delimiter-error {
        extra(tuple<char, span>),
        missing(tuple<char, span>),
    }

    repair: func(errors: list<delimiter-error>, source: string) -> string;
}

world plugin {
    export repair;
}
```
