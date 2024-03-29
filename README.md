# anxious

![static analysis](https://github.com/mspiegel/anxious/actions/workflows/static-analysis.yaml/badge.svg) ![unit tests](https://github.com/mspiegel/anxious/actions/workflows/unit-tests.yaml/badge.svg) [![codecov](https://codecov.io/gh/mspiegel/anxious/graph/badge.svg?token=7H2EY41PIE)](https://codecov.io/gh/mspiegel/anxious)

Anxious has two kinds of types:

Nominal types. Nominal types represent values that are definitely not errors. The nominal types are what you would typically consider to be the types in other programming languages. Only total functions are defined on nominal types. Partial functions use axious types.

Anxious types. Anxious types are the default types of the Panic type system. Anxious types are defined as sum types (tagged union) of nominal types and the Panic enum.

#### Integer Operations

| Operation           | I8  | INom8 | IBit8 | INomBit8 |
| ------------------- | --- | ------- | -- | ------- |
| `+`, `-`, `*`       | ✓  |          | ✓  | ✓      |
| `/`, `%`            | ✓  |          | ✓  |        |
| `==`, `!=`          |    | ✓        |    | ✓      |
| `<`, `>`            |    | ✓        |    | ✓      |
| `<=`, `>=`          |    | ✓        |    | ✓      |
| `&`, `\|`, `^`      |    |          | ✓  | ✓      |
| `!`                 |    |          | ✓  | ✓      |
| `<<`, `>>`          |    |          | ✓  | ✓      |
| Display             |    | ✓        |    | ✓      |
| Debug               | ✓  | ✓        | ✓ | ✓      |

`I8` can return Panic value `IntegerOverflow` on arithmetic operations `+`, `-`, `*`  
`I8` can return Panic value `IntegerOverflow` or `IntegerDivizionByZero` on arithmetic operations `/`, `%`  
`IBit8` and `INomBit8` perform wrapped arithmetic on `+`, `-`, `*`  
`IBit8` can return Panic value `IntegerDivizionByZero` on arithmetic operations `/`, `%`  
Equality and Comparison are defined on the nominal types  
Display is defined on all nominal types  
Debug is defined on all types  
