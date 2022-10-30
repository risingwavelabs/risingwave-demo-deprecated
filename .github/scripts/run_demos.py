#!/usr/bin/python3

from os.path import (dirname, abspath)
import os
import subprocess
from time import sleep
import sys


redpanda_smp = "REDPANDA_SMP_NUM={}".format(round(os.cpu_count()/2+1))
print(redpanda_smp)


def run_sql_file(f: str, dir: str):
    print("Running SQL file: {}".format(f))
    # ON_ERROR_STOP=1 will let psql return error code when the query fails.
    # https://stackoverflow.com/questions/37072245/check-return-status-of-psql-command-in-unix-shell-scripting
    subprocess.run(["psql", "-h", "localhost", "-p", "4566",
                    "-d", "dev", "-U", "root", "-f", f, "-v", "ON_ERROR_STOP=1"],
                   cwd=dir, check=True, capture_output=True)


def run_demo(demo: str):
    file_dir = dirname(abspath(__file__))
    project_dir = dirname(dirname(file_dir))
    demo_dir = os.path.join(project_dir, demo)
    print("Running demo: {}".format(demo))

    f = open("{}/{}".format(demo_dir, ".env"), "w")
    f.write(redpanda_smp)
    f.close()

    subprocess.run(["docker", "compose", "up", "-d"],
                   cwd=demo_dir, check=True)
    sleep(40)

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


run_demo(sys.argv[1])
