CREATE SOURCE twitter WITH (
    connector = 'kafka',
    kafka.topic = 'twitter',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT AVRO MESSAGE 'Event' ROW SCHEMA LOCATION 'http://file_server:8080/avro.json';