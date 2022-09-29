CREATE SOURCE prometheus (
    labels STRUCT < __name__ VARCHAR,
    instance VARCHAR,
    job VARCHAR >,
    name VARCHAR,
    timestamp TIMESTAMP,
    value VARCHAR
) WITH (
    connector = 'kafka',
    kafka.topic = 'prometheus',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;