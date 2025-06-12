#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

set -euxCo pipefail

scriptDir=$(cd "$(dirname "$0")" && pwd)
cd "$scriptDir"

autocast --overwrite demo.yaml demo.cast
agg --font-family "Cascadia Code,Hack,Source Code Pro" demo.cast demo.gif
gifsicle -b -O3 demo.gif
