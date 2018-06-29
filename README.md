# dtparse
[![Build
 Status](https://travis-ci.org/markcol/dtparse.svg?branch=master)](https://travis-ci.org/markcol/dtparse)
 [![License:
 MIT](https://img.shields.io/badge/License-Apache-2.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0) [![Coverage Status](https://coveralls.io/repos/github/markcol/dtparse/badge.svg?branch=master)](https://coveralls.io/github/markcol/dtparse?branch=master)

A [dateutil]-compatible timestamp parser for Rust.

## Where it stands

The library works really well at the moment, and passes the vast
majority of `dateutil`s parser test suite. This isn't mission-critical
ready, but is more than ready for hobbyist projects.

The issues to be resolved before version 1.0:

**Functionality**:

1. ~~We don't support weekday parsing. In the Python side this is
accomplished via `dateutil.relativedelta`~~ Supported in v0.8

1. Named timezones aren't supported very well. [chrono_tz]
theoretically would provide support, but I'd also like some helper
things available (e.g. "EST" is not a named zone in `chrono-tz`).
Explicit time zones (i.e. "00:00:00 -0300") are working as expected.

1. "Fuzzy" and "Fuzzy with tokens" modes haven't been tested. The code
should work, but I need to get the test cases added to the
auto-generation suite

**Non-functional**: This library is intended to be a direct port from
Python, and thus the code looks a lot more like Python than it does
Rust. There are a ton of `TODO` comments in the code that need cleaned
up, things that could be converted to enums, etc.

In addition, some more documentation would be incredibly helpful.
It's, uh, sparse at the moment.

[dateutil]: https://github.com/dateutil/dateutil
[crono_tz]: https://github.com/cronotope/chrono-tz
