# tree-sitter-csv

[![Build Status](https://github.com/dpezto/tree-sitter-csv/actions/workflows/ci.yml/badge.svg)](https://github.com/dpezto/tree-sitter-csv/actions/workflows/ci.yml)
[![Discord](https://img.shields.io/discord/1063097320771698699?logo=discord)](https://discord.gg/w7nTvsVJhm)

CSV, PSV, and TSV grammars for [tree-sitter](https://github.com/tree-sitter/tree-sitter).

Because CSV, PSV, and TSV are actually three different dialects, this module defines
three grammars. Require them as follows:

```js
require("tree-sitter-csv").csv; // CSV grammar
require("tree-sitter-csv").psv; // PSV grammar
require("tree-sitter-csv").tsv; // TSV grammar
```

Fields are classified as `text`, `number` (integers and hexadecimal literals),
`float` (including scientific notation), `boolean` (`true` and `false`), or `date`.
Decimal numbers support signs and `_` between digits. Quoted fields remain `text`,
and empty fields retain their column positions.

Dates recognize numeric day/month/year and year/month/day forms with `-`, `/`,
or `.`, optionally followed by a time separated by a space or `T`. Times support
optional seconds, fractional seconds, AM/PM, and `Z` or numeric UTC offsets
(`-06:00` or `-0600`). Recognition is syntactic: dates such as `2025-02-31` can
still be classified as `date`. Calendar validity and ambiguous date order belong
to the consuming application. Compact dates such as `20261101` remain numbers.

Run `npm install`, `npm run build`, and `npm test` to regenerate and test all three
grammars. Parsers target ABI 15 and require a Tree-sitter 0.25 or newer runtime.

## Layout

`common/` holds the shared grammar and empty-field scanner. Each dialect keeps its
entry point, generated C sources, and queries in `csv/`, `psv/`, or `tsv/`.
`test/corpus/` exercises all three grammars. Node, Rust, and Swift expose all three
languages; CMake, Make, Go, and Python currently expose CSV only.

Generated parser C sources and headers are versioned so editors can compile the
parsers without running the generator. Compiled libraries and build caches are
ignored. Run `npm run build` after editing the shared grammar.

## Neovim

For the `main` branch of `nvim-treesitter`, register all three dialects with their
query directories. Push these commits to your fork before using the remote URL.

```lua
vim.api.nvim_create_autocmd("User", {
  pattern = "TSUpdate",
  callback = function()
    local parsers = require("nvim-treesitter.parsers")
    for _, lang in ipairs({ "csv", "psv", "tsv" }) do
      parsers[lang].install_info = {
        url = "https://github.com/dpezto/tree-sitter-csv",
        branch = "master",
        location = lang,
        queries = lang .. "/queries",
        generate = false,
      }
    end
  end,
})
```

For local development, replace `url` and `branch` with
`path = vim.fn.expand("~/Developer/tools/nvim/tree-sitter-csv")`.
Then restart Neovim and run `:TSInstall! csv psv tsv` to rebuild the parsers and
install the matching highlighting queries. Restart Neovim again after installation
to refresh any grammars that were already loaded.
