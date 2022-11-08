# Clippy vs Raw Strings

Currently, the rust linter Clippy has an issue with unicode in raw strings: some lints will try to replace literals with escape sequences, which raw strings don't support.

This breakage may potentially be caught by the compiler, however, when a raw string is used as a format string.


This repo should demonstrate some cases. Example string contents are taken from the Clippy lint examples.
