# recwave

A PoC Recommender System that Utilizes RisingWave

## Installation

To prepare an environment for building and running Recwave, go to the root directory of Recwave and execute the following

```
docker build -t recwave-env .
```

The container will include a RisingWave binary, Kafka and some other system libraries necessary for Recwave. Running this image will open a bash terminal as default entrypoint.

```
docker run -it recwave-env
```

To build and run Recwave inside this docker environment, mount the Rust build directory and the Recwave source directory upon running the docker image.

```
# backup the host cargo environment
cp -r $HOME/.cargo $HOME/recwave/cargo

# start recwave-env with existing cargo environment
docker run -it -v $(pwd):/opt/recwave -v $HOME/recwave/cargo:/root/.cargo recwave-env
```

## What's included in the container

* Infrastructure
    * RisingWave
    * Kafka Cluster

* Recommender Pipeline
    * Build Dependencies: librdkafka, pkg-config , openssl
  
* User Simulator

* Generator & Recommender Model
    * Runtime: Python 3
    * Requirements: numpy, psycopg, click
