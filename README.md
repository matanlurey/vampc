# vamp

A safe, deterministic, and simple configuration language written in Rust.

## Prior Art

There is almost no limit to the number of configuration languages that already
present themselves as language-agnostic or domain specific configuration
langauges. Below is a non-exhaustive lists of the popular ones, and why I think
they are/are not a good fit for my projects.

> NOTE: It turns out you can use most programming languages as configuration
> langauges, such as [Python][] or even [Scala][]. However, both of these
> options break something important to me, which is limiting the ability to
> interact with the platform.\
>
> Being entirely [deterministic][] is an important quality to me, because it
> means the configuration language could be easily adapted for caching
> scenarios or reproducible builds.
>
> \*_You could write a sophisticated linter or custom runtime to prevent usage
> of I/O, but then (a) why would I make this project, and (b) why not just
> use something off the shelf like [Starlark][]._

[deterministic]: https://en.wikipedia.org/wiki/Deterministic_algorithm
[python]: https://www.lucidchart.com/techblog/2018/07/16/why-json-isnt-a-good-configuration-language/
[scala]: https://medium.com/@cvogt/scala-as-a-configuration-language-f075b058a660

### Markup Languages

These are simple languages that mostly are different ways to represent a
_[Dictionary][]-like_ (frequently referred to as a HashMap or Table), and are
sometimes suitable for _encoding_. They are among the most popular, mostly for
being simple, having implementations in nearly every langauge, and not allowing
code execution.

- [JSON][]: JavaScript Object Notation.

  - ✔️ Probably the most common/popular "configuration language" today.
  - ⚠️ Decent at being human readable and machine writeable.
  - ❌ Doesn't support comments.
  - ❌ No easy way to semantically validate aside from [JSON Schema][].

- [YAML][]: YAML Ain't Markup Language.

  - ✔️ Very easy to read.
  - ✔️ Supports comments.
  - ❌ Not typically machine writeable.
  - ❌ Whitespace significant in extremely confusing ways.
  - ❌ Resulting parsed type is _highly_ context sensitive.
  - ❌ Good luck validating.

- [TOML][]: Tom's Obvious, Minimal Language.

  - ✔️ Very easy to read.
  - ✔️ Supports comments.
  - ⚠️ Nested dictionaries don't look as great.
  - ⚠️ Decent at being machine writeable.
  - ❌ No standard way to validate.

[dictionary]: https://en.wikipedia.org/wiki/Associative_array
[json]: https://www.json.org/json-en.html
[json schema]: https://json-schema.org/
[yaml]: https://yaml.org/
[toml]: https://github.com/toml-lang/toml

### Attempts to Improve JSON

Given JSON's wild popularity (mostly due to the browser and NodeJS, not
inherently due to JSON being the best by any means), there are/were various
attempts to try and "bolt-on" additional features and create a superset of JSON.

- [Hjson][]: A user interface for JSON.

  - ✔️ Very easy to read (even easier than JSON or TOML, IMO).
  - ✔️ Allows comments, omits a lot of punctuation.
  - ⚠️ "Community supported" (interwebs lingo for not supported).
  - ⚠️ Still lets you express JSON with all the problems of JSON.
  - ❌ No easy way to semantically validate aside from [JSON Schema][].

- [Hocon][]: Human-Optimzied Config Object Notation.

  - ✔️ Very easy to read (even easier than JSON).
  - ✔️ Adds extensions making it easier to write than JSON.
  - ✔️ Allows comments, omits some punctuation.
  - ✔️ Adds `include` statements.
  - ⚠️ Include semantics are suprisingly confusing.
  - ⚠️ You can include a `url()`.
  - ❌ Implements "may" supported additional extended syntax.
  - ❌ No easy way to semantically validate aside from [JSON Schema][].

- [Jsonnet][]: A "simple" extension of JSON.

  - ✔️ Adds extensions making it easier to write than JSON.
  - ✔️ Allows comments, omits some punctuation.
  - ✔️ Adds variables, conditionals, functions, imports, etc.
  - ⚠️ Loses some of the ability to make it easy to read/understand.
  - ❌ Fairly bizarre standard library.
  - ❌ No easy way to semantically validate aside from [JSON Schema][].

[hjson]: https://hjson.github.io/
[hocon]: https://github.com/lightbend/config/blob/master/HOCON.md
[jsonnet]: https://jsonnet.org/

### Domain-specific Languages

Going outside JSON and simple dictionary languages are langauges written for
the most part to be excellent configuration langauges (or in some cases, like
Starlark, just what happens when you throw enough Google engineers and Python
together for a decade).

These languages are closer to what I'd like `Vamp` to become!

- [Dhall][]: Maintainable configuration files.

  - ✔️ Finally, some type checking.
  - ✔️ Built-in tooling and conversions.
  - ✔️ Not Turing-complete, forbids side-effects.
  - ⚠️ Missing some modern concepts, like immutablility.

- [Nix][]: Nix Expression Langauge.

  - ✔️ Pure, lazy, functional.
  - ⚠️ Virtually no tooling (at least compared to Dhall).
  - ❌ No easy way to semantically validate.

- [Meson][]: Build configuration language.

  - 🙃 Might as well just evaluate Starlark they are so similar.

- [Starlark][]: A python-like language used by [Bazel][].

  - ✔️ Built-in tooling and conversions.
  - ✔️ Not Turing-complete, forbids side-effects.
  - ⚠️ Writing Python sucks.
  - ❌ No easy way to semantically validate.

[dhall]: https://dhall-lang.org/
[nix]: https://nixos.wiki/wiki/Nix_Expression_Language
[meson]: https://mesonbuild.com/Syntax.html
[starlark]: https://docs.bazel.build/versions/master/skylark/language.html
[bazel]: https://www.bazel.build/

## What to build

Based on the above [prior art](#prior-art), I want to build a simple but
expressive configuration language that leverages some of the best things about
markup langauges (determistic final result, e.g. a `Dictionary`) and a more
traditional programming langauge, with the following considerations:

- The end result of evaluating should be deterministic.
- It should be easy to represent the end result as JSON or another format.
- By default it should be impossible for configuration to have side effects.
- The language should feel terse, simple, and modern and support [DRY][].
- Simple but optional static type checking of the result.
- A nice set of tooling for converting, refactoring, linting, etc.

[dry]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself

### Examples

Define an object with a single key, `name`, whose value is `'Matan'`:

```txt
out {
  name = 'Matan'
}
```

... emits:

```json
{
  "name": "Matan"
}
```

---

Define an array with different color Corgis:

```txt
colors = ['Red', 'Sable', 'Black']

out {
  dogs = colors.map(c => '${c} Corgi')
}
```

... emits:

```json
{
  "dogs": ["Red Corgi", "Sable Corgi", "Black Corgi"]
}
```
