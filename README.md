
<!-- README.md is generated from README.Rmd. Please edit that file -->

# {fio}

<div style="text-align: justify">

Friendly & Fast Input-Output Analysis
<img src="man/figures/leontief.jpg" align="right" width="240px" style="margin-left: 20px;" />

<!-- badges: start -->

[![R-CMD-check](https://github.com/albersonmiranda/fio/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/albersonmiranda/fio/actions/workflows/R-CMD-check.yaml)
[![CRAN
status](https://www.r-pkg.org/badges/version/fio)](https://CRAN.R-project.org/package=fio)
[![Codecov test
coverage](https://codecov.io/gh/albersonmiranda/fio/branch/main/graph/badge.svg)](https://app.codecov.io/gh/albersonmiranda/fio?branch=main)
<!-- badges: end -->

`{fio}` (*Friendly Input-Output*) is a R package designed for
input-output analysis, emphasizing usability for Excel users and
performance. It includes an [RStudio
Addin](https://rstudio.github.io/rstudioaddins/) and a suite of
functions for straightforward import of input-output tables from Excel,
either programmatically or directly from the clipboard.

The package is optimized for speed and efficiency. It leverages the [R6
class](https://r6.r-lib.org/) for clean, memory-efficient
object-oriented programming. Furthermore, all linear algebra
computations are implemented in [Rust](https://www.rust-lang.org/) to
achieve highly optimized performance.

## Installation

You can install the released version of `{fio}` from CRAN with:

``` r
install.packages("fio")
```

For macOS and Linux, or if you wish to compile the latest development
version from source, you will need to have
[Rust](https://www.rust-lang.org/) installed on your system.

- Debian/Ubuntu: `apt-get install cargo`
- Fedora/CentOS: `dnf install cargo`
- macOS: `brew install rustc`
- Windows: <https://www.rust-lang.org/tools/install>

You’ll also find previous releases of `fio` in the [Releases
section](https://github.com/albersonmiranda/fio/releases) on GitHub, and
precompiled binaries for Windows, macOS, and Linux at
[R-Universe](https://albersonmiranda.r-universe.dev/fio)

## Getting Started

If you are just getting started with `{fio}`, we recommend you to read
the
[vignettes](https://albersonmiranda.github.io/fio/articles/index.html)
for a comprehensive overview of the package.

## Examples

Calculate Leontief’s inverse from brazilian 2020 input-output matrix:

``` r
# load included dataset
iom_br <- fio::br_2020

# limit threads to 1 for CRAN checks
iom_br$set_max_threads(1)

# calculate technical coefficients matrix
iom_br$compute_tech_coeff()

# calculate Leontief's inverse
iom_br$compute_leontief_inverse()
```

And pronto! 🎉, you’re all good to carry on with your analysis. You can
evoke the Data Viewer to inspect the results with
`iom_br$technical_coefficients_matrix |> View()` and
`iom_br$leontief_inverse_matrix |> View()`.

![](man/figures/example_leontief_inverse.png) *<small>Leontief’s inverse
from brazilian 2020 input-output matrix</small>*

## Related tools

Other great tools for input-output analysis in R include:

- [{leontief}](https://pachamaltese.github.io/leontief/)
- [{ioanalysis}](https://cran.r-project.org/package=ioanalysis)

</div>
