# Documentation

Documentation is stored in the `docs` folder.

## API docs

Only documents `pub` items — this is what consumers of the crate see.

```bash
cargo docs
```

## This developer guide

Requires `mdbook` and `mdbook-pdf`. To create documentation use:

```bash
cargo xtask docs (--open) # builds docs/book/html and docs/book/pdf/output.pdf
```

Chapters are listed in `docs/src/SUMMARY.md`; add a file there to add a page.

## Experimental PDF (Eisvogel)

To improve quality we experiment with eisvogel crate. Following command renders
the same chapters through Pandoc + the [Eisvogel](https://github.com/Wandmalfarbe/pandoc-latex-template) LaTeX
template. It requires needs `pandoc`, `xelatex`, and the Eisvogel template.

Output goes to `docs/book/pdf-eisvogel/output.pdf`. Not wired into CI yet.

```bash
cargo xtask docs --eisvogel
```

