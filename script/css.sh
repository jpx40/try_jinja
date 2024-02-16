#! /bin/bash

rm -rf static/style/*
lightningcss --minify --bundle --targets '>=0.25%' style/* --output-dir static/style/bundle.css
