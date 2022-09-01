#!/bin/bash
# run docker container locally (for testing)
# assumes you're running in the backend/ dir of the project

docker run -i --entrypoint /backend/entrypoint.sh -v $PWD:/backend fantastic_backend
mkdir -p ../bin
cp ./out/backend ../bin
