#!/bin/bash
k6 run --vus 1000 --duration 30s ./test.js
