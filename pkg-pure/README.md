# `@silicon-soldier/erase-ts-types`

Erases types from TypeScript source leaving behind plain JS, in a very litteral sense. TypeScript constructs such as `enum` _may_ end up usable but are not supported, features that rely on other files (e.g. `const enum`) won't work as this relies on looking up values in other files. Adding the isolated modules flag to type checks should flush out most of the issues, the rest tests and bundlers should be able to catch when they encounter nonsense code.

Behind the scenes this is implemented in WASM compiled from Rust, using `swc` libraries for the leg work.

## Usage

```js
import { erase } from "@silicon-soldier/erase-ts-types";
console.log(erase(`
interface Foo {}
export class Bar implements Foo {}
`));
// export class Bar {}
```
