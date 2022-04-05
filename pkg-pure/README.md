# `@silicon-soldier/erase-ts-types`

Erases types from TypeScript source, leaving behind plain JS. No type checks are performed nor any lookups, so make sure input source can be compiled in isolation (no `const enums`).

## Usage

```js
import { erase } from "@silicon-soldier/erase-ts-types";
console.log(erase(`
interface Foo {}
export class Bar implements Foo {}
`));
// export class Bar {}
```
