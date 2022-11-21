#!/usr/bin/python3

import argparse
import os
import sys
from os.path import (dirname, abspath)

datagen = """
  datagen:
    image: ghcr.io/risingwavelabs/demo-datagen:v1.0.8
    depends_on: [message_queue]
    command:
      - /bin/sh
      - -c
      - /datagen --mode {} --qps 2 kafka --brokers message_queue:29092
    restart: always
    container_name: datagen
"""

message_queue = """  message_queue:
    image: docker.redpanda.com/vectorized/redpanda:latest
    command:
      - redpanda start
      - --smp 1
      - --overprovisioned
      - --node-id 0
      - --kafka-addr PLAINTEXT://0.0.0.0:29092,OUTSIDE://0.0.0.0:9092
      - --advertise-kafka-addr PLAINTEXT://message_queue:29092,OUTSIDE://localhost:9092
    ports:
      - 29092:29092
    container_name: message_queue
    healthcheck:
      test:
        - CMD
        - printf
        - ""
        - /dev/tcp/127.0.0.1/9092
      interval: 1s
      timeout: 5s
      retries: 5"""

file_server = """  file_server:
    image: halverneus/static-file-server:latest
    volumes:
      - "./schema:/schema"
    restart: always
    environment:
      FOLDER: /
    container_name: file_server
"""

additional_volumes = """  message_queue:
    external: false
"""


def gen_docker_compose(demo: str, format: str, base_compose: str) -> str:
    content = ""
    with open(base_compose) as file:
        for line in file:
            if line == 'volumes:\n':
                content += message_queue
                content += gen_datagen(demo, format)
            if line == "name: risingwave-compose\n":
                content += additional_volumes
            content += line
    return content


def gen_datagen(demo: str, format: str) -> str:
    if format == 'json':
        return gen_json_datagen(demo)
    elif format == 'protobuf':
        return gen_pb_datagen(demo)
    else:
        print('Unknown format: {}'.format(format))
        sys.exit(1)


def gen_json_datagen(demo: str) -> str:
    return datagen.format(demo)


def gen_pb_datagen(demo: str) -> str:
    return datagen.format(demo) + file_server


arg_parser = argparse.ArgumentParser(
    description='Generate the docker compose file for every test cases')
arg_parser.add_argument('--format',
                        metavar='format',
                        type=str,
                        help='the format of output data')
arg_parser.add_argument('--case',
                        metavar='case',
                        type=str,
                        help='the test case')
args = arg_parser.parse_args()

if args.case == 'docker':
    print('Will not generate docker-compose file for `docker`')
    sys.exit(0)

file_dir = dirname(abspath(__file__))
project_dir = dirname(dirname(file_dir))
demo_dir = os.path.join(project_dir, args.case)
base_compose = os.path.join(os.path.join(
    project_dir, 'docker'), 'docker-compose.yml')
demo_compose = os.path.join(demo_dir, 'docker-compose.yml')

content = gen_docker_compose(args.case, args.format, base_compose)
with open(demo_compose, 'w') as file:
    file.write(content)
