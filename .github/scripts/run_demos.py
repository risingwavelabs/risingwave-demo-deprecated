#!/usr/bin/python3

from os.path import (dirname, abspath)
import os
import subprocess
from time import sleep


demos = [
    "ad-click",
    "ad-ctr",
    "cdn-metrics",
    "clickstream",
    # "delivery",
    # "ecommerce",
    "live-stream",
    "twitter",
    # "twitter-pulsar"
]


def run_sql_file(f: str, dir: str):
    print("Running SQL file: {}".format(f))
    # ON_ERROR_STOP=1 will let psql return error code when the query fails.
    # https://stackoverflow.com/questions/37072245/check-return-status-of-psql-command-in-unix-shell-scripting
    subprocess.run(["psql", "-h", "localhost", "-p", "4566",
                    "-d", "dev", "-U", "root", "-f", f, "-v", "ON_ERROR_STOP=1"], cwd=dir, check=True, capture_output=True)


def run_demo(demo: str):
    file_dir = dirname(abspath(__file__))
    project_dir = dirname(dirname(file_dir))
    demo_dir = os.path.join(project_dir, demo)
    print("Running demo: {}".format(demo))

    subprocess.run(["docker", "compose", "up", "-d"], cwd=demo_dir, check=True)
    sleep(30)

    sql_file = os.path.join(demo_dir, "create_source.sql")
    run_sql_file(sql_file, demo_dir)
    sleep(10)

    sql_file = os.path.join(demo_dir, "create_mv.sql")
    run_sql_file(sql_file, demo_dir)
    sleep(10)

    sql_file = os.path.join(demo_dir, "query.sql")
    run_sql_file(sql_file, demo_dir)
    sleep(10)

    subprocess.run(["docker", "compose", "down"], cwd=demo_dir, check=True)


for demo in demos:
    run_demo(demo)
