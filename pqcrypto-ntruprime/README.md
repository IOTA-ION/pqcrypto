# ntruprime


This crate contains bindings to the C implementations of the following schemes,
from [PQClean][pqclean].

This project packages Post-Quantum cryptographic algorithms that participate in
the [NIST PQC standardization effort][nistpqc]. It is currently a collection of
wrappers around C implementations from the [PQClean][pqclean] project.

## Serialization

If you want `serde` support, enable the `serialization` feature.

## Included implementations from PQClean

Below is a list of the included schemes and the corresponding implementations
sourced from [PQClean][pqclean]. The "default" implementation is used in the
Rust-friendly interface, alternative implementations are exposed as ``ffi``
methods only.

 * ``ntrulpr653``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``ntrulpr761``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``ntrulpr857``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``ntrulpr953``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``ntrulpr1013``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``ntrulpr1277``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup653``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup761``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup857``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup953``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup1013``
    * ``avx2`` (if supported)
    * ``clean`` (default)
 * ``sntrup1277``
    * ``avx2`` (if supported)
    * ``clean`` (default)


## License

The wrappers and wrapper generation scripts in this project are covered by the
MIT or Apache 2.0 licenses, at your choice.

The implementations we link to are not, however. Please see the [PQClean][pqclean]
project for the appropriate licenses.

[pqclean]: https://github.com/PQClean/PQClean/
[nistpqc]: https://nist.gov/pqc/