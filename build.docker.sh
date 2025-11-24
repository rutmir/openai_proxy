#!/bin/bash

docker rmi rutmir/openai-proxy-carousel
docker build -t rutmir/openai-proxy-carousel .
