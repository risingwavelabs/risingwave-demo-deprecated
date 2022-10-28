--
-- The Kafka source version
--
CREATE SOURCE twitter (
    data STRUCT < created_at TIMESTAMPTZ,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMPTZ,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >
) WITH (
    connector = 'kafka',
    kafka.topic = 'twitter',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;