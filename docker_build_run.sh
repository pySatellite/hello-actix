#!/bin/bash

ver=$(cat Cargo.toml | grep "version" | cut -d "=" -f 2 | sed 's/"//g' | sed 's/ //g')
ver_num=$(echo $ver | sed 's/\.//g')

docker build -t pysatellite/hello-actix:$ver .

docker stop hello-actix
docker run -dit --rm --name hello-actix -p 8${ver_num}:8080 --cpus=".01" --memory 6m pysatellite/hello-actix:$ver

docker stats
