CREATE SOURCE prometheus (
    labels STRUCT < __name__ VARCHAR,
    event VARCHAR,
    instance VARCHAR,
    job VARCHAR,
    role VARCHAR >,
    name VARCHAR,
    timestamp VARCHAR,
    value VARCHAR
) WITH (
    connector = 'kafka',
    kafka.topic = 'prometheus',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;