#! /bin/bash

lightningcss --minify --bundle --targets '>=0.25%' ./style/* --output-file ./static/style/bundle.css
