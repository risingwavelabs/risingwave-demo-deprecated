main(){
  # install kafka and zookeeper
  if ! which brew &> /dev/null
  then
    echo "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  fi

  echo "Checking kafka and zookeeper"

  which kafka-server-start &> /dev/null || brew install kafka
  create_zookeeper_config
  create_kafka_config

  # start kafka and zookeeper
  zookeeper-server-start ./zookeeper.properties > /dev/null &
  kafka-server-start ./server.properties > /dev/null &
  sleep 1

  # check if successful
  if ! nc -z localhost 9092 || ! nc -z localhost 2181
  then
    echo "Run Kafka server failed, please check kafka and etcd port"
    exit 1
  fi
  echo "Kafka cluster started successfully"

  # set kafka topic
  kafka-topics --create --topic recwave --partitions 1 --replication-factor 1 --if-not-exists --bootstrap-server localhost:9092

  # build and start risingwave cluster
  cd $1 && ./risedev d &
  psql -h localhost -p 4566 -d dev -U root -a -f recwave-start.sql

  # build
  (cargo build --manifest-path=./recommender/Cargo.toml --release) || clean_up; exit 1
  (cargo build --manifest-path=./simulator/Cargo.toml --release) || clean_up; exit 1
  python3 generator
  python3 recommender/model &
  ./recommender/target/release/recommender &
  ./simulator/target/release/simulator &
}

create_zookeeper_config() {
  echo "# the directory where the snapshot is stored.
        dataDir=/tmp/zookeeper
        # the port at which the clients will connect
        clientPort=2181
        # disable the per-ip limit on the number of connections since this is a non-production config
        maxClientCnxns=0
        # Disable the adminserver by default to avoid port conflicts.
        # Set the port to something non-conflicting if choosing to enable this
        admin.enableServer=false
        # admin.serverPort=8080" > ./zookeeper.properties
}

create_kafka_config() {
  echo "# The id of the broker. This must be set to a unique integer for each broker.
  broker.id=0

  ############################# Socket Server Settings #############################

  # The address the socket server listens on. If not configured, the host name will be equal to the value of
  # java.net.InetAddress.getCanonicalHostName(), with PLAINTEXT listener name, and port 9092.
  #   FORMAT:
  #     listeners = listener_name://host_name:port
  #   EXAMPLE:
  #     listeners = PLAINTEXT://your.host.name:9092
  listeners=PLAINTEXT://127.0.0.1:9092

  # Listener name, hostname and port the broker will advertise to clients.
  # If not set, it uses the value for 'listeners'.
  #advertised.listeners=PLAINTEXT://your.host.name:9092

  # Maps listener names to security protocols, the default is for them to be the same. See the config documentation for more details
  #listener.security.protocol.map=PLAINTEXT:PLAINTEXT,SSL:SSL,SASL_PLAINTEXT:SASL_PLAINTEXT,SASL_SSL:SASL_SSL

  # The number of threads that the server uses for receiving requests from the network and sending responses to the network
  num.network.threads=3

  # The number of threads that the server uses for processing requests, which may include disk I/O
  num.io.threads=8

  # The send buffer (SO_SNDBUF) used by the socket server
  socket.send.buffer.bytes=102400

  # The receive buffer (SO_RCVBUF) used by the socket server
  socket.receive.buffer.bytes=102400

  # The maximum size of a request that the socket server will accept (protection against OOM)
  socket.request.max.bytes=104857600


  ############################# Log Basics #############################

  # A comma separated list of directories under which to store log files
  log.dirs=/tmp/kafka-logs

  # The default number of log partitions per topic. More partitions allow greater
  # parallelism for consumption, but this will also result in more files across
  # the brokers.
  num.partitions=1

  # The number of threads per data directory to be used for log recovery at startup and flushing at shutdown.
  # This value is recommended to be increased for installations with data dirs located in RAID array.
  num.recovery.threads.per.data.dir=1

  ############################# Internal Topic Settings  #############################
  # The replication factor for the group metadata internal topics '__consumer_offsets' and '__transaction_state'
  # For anything other than development testing, a value greater than 1 is recommended to ensure availability such as 3.
  offsets.topic.replication.factor=1
  transaction.state.log.replication.factor=1
  transaction.state.log.min.isr=1

  ############################# Log Flush Policy #############################

  # Messages are immediately written to the filesystem but by default we only fsync() to sync
  # the OS cache lazily. The following configurations control the flush of data to disk.
  # There are a few important trade-offs here:
  #    1. Durability: Unflushed data may be lost if you are not using replication.
  #    2. Latency: Very large flush intervals may lead to latency spikes when the flush does occur as there will be a lot of data to flush.
  #    3. Throughput: The flush is generally the most expensive operation, and a small flush interval may lead to excessive seeks.
  # The settings below allow one to configure the flush policy to flush data after a period of time or
  # every N messages (or both). This can be done globally and overridden on a per-topic basis.

  # The number of messages to accept before forcing a flush of data to disk
  #log.flush.interval.messages=10000

  # The maximum amount of time a message can sit in a log before we force a flush
  #log.flush.interval.ms=1000

  ############################# Log Retention Policy #############################

  # The following configurations control the disposal of log segments. The policy can
  # be set to delete segments after a period of time, or after a given size has accumulated.
  # A segment will be deleted whenever *either* of these criteria are met. Deletion always happens
  # from the end of the log.

  # The minimum age of a log file to be eligible for deletion due to age
  log.retention.hours=168

  # A size-based retention policy for logs. Segments are pruned from the log unless the remaining
  # segments drop below log.retention.bytes. Functions independently of log.retention.hours.
  #log.retention.bytes=1073741824

  # The maximum size of a log segment file. When this size is reached a new log segment will be created.
  log.segment.bytes=1073741824

  # The interval at which log segments are checked to see if they can be deleted according
  # to the retention policies
  log.retention.check.interval.ms=300000

  ############################# Zookeeper #############################

  # Zookeeper connection string (see zookeeper docs for details).
  # This is a comma separated host:port pairs, each corresponding to a zk
  # server. e.g. '127.0.0.1:3000,127.0.0.1:3001,127.0.0.1:3002'.
  # You can also append an optional chroot string to the urls to specify the
  # root directory for all kafka znodes.
  zookeeper.connect=localhost:2181

  # Timeout in ms for connecting to zookeeper
  zookeeper.connection.timeout.ms=18000


  ############################# Group Coordinator Settings #############################

  # The following configuration specifies the time, in milliseconds, that the GroupCoordinator will delay the initial consumer rebalance.
  # The rebalance will be further delayed by the value of group.initial.rebalance.delay.ms as new members join the group, up to a maximum of max.poll.interval.ms.
  # The default value for this is 3 seconds.
  # We override this to 0 here as it makes for a better out-of-the-box experience for development and testing.
  # However, in production environments the default value of 3 seconds is more suitable as this will help to avoid unnecessary, and potentially expensive, rebalances during application startup.
  group.initial.rebalance.delay.ms=0
  " > ./kafka.properties
}

clean_up (){
  kafka-server-stop
  rm ./kafka.properties
  rm ./zookeeper.properties
}

main "$@"
clean_up