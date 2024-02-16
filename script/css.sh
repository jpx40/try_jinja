#! /bin/bash

lightningcss --minify --bundle --targets '>=0.25%' style/* --output-dir static/style/bundle.css
