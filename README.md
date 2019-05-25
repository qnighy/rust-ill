## Representation of ILL (Intuitionistic Linear Logic) in Rust

This is a playground where we can confirm Intuitionistic Linear Logic over Rust.
See e.g. [a Wikipedia article](https://en.wikipedia.org/wiki/Linear_logic) or [LLWiki](http://llwiki.ens-lyon.fr/mediawiki/index.php/Main_Page) for Linear Logic.

## Types

- An atomic proposition is a type parameter with the bounds `Sized + 'static`.
- linear implication
  - `Limpl<A, B>` = A ⊸ B
- multiplicative conjunction
  - `One` = 1
  - `Tensor<A, B>` = A ⊗ B
- additive conjunction
  - `Top` = ⊤
  - `With<A, B>` = A & B
- additive disjunction
  - `Zero` = 0
  - `Plus<A, B>` = A ⊕ B
- exponential conjunction
  - `Bang<A>` = !A

Connectives like (-)<sup>⊥</sup>, ⅋, ⊥, and `?` are out of the Intuitionistic fragment.

## Rules

- No panics (e.g. `panic!`, `unreachable!`)
- No exits (e.g. `abort`, `exit`)
- No infinite loops or infinite recursions
- No unsafes (especially `mem::transmute`s)
- No drops other than `Bang<_>` and `One`
- No forgets (either through `mem::forget` or through `Arc` cycles)
