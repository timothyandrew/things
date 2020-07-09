#!/usr/bin/env bash

wasm-pack build --target web

# World's most advanced build system!
rm -r ~/me/static/wasm/things && mv pkg ~/me/static/wasm/things