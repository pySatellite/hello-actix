#!/bin/bash

ver=$(cat Cargo.toml | grep "version" | cut -d "=" -f 2 | sed 's/"//g' | sed 's/ //g')

docker build -t pysatellite/hello-actix:$ver .

docker tag pysatellite/hello-actix:$ver pysatellite/hello-actix

docker push pysatellite/hello-actix


echo '                     ##        .
              ## ## ##       ==
           ## ## ## ##      ===
       /""""""""""""""""\___/ ===
  ~~~ {~~ ~~~~ ~~~ ~~~~ ~~ ~ /  ===- ~~~
       \______ o          __/
         \    \        __/
          \____\______/'