#!/bin/sh

set -euxo pipefail

cargo test
wasm-pack build wasm --target web

scp -P 10239 wasm/index.html  wkarpiel@maluch2.mikr.us:/home/wkarpiel/dokumenty/wojciechkarpiel-pl-static/beciarz/index.html
scp -P 10239 -r wasm/pkg wkarpiel@maluch2.mikr.us:/home/wkarpiel/dokumenty/wojciechkarpiel-pl-static/beciarz/