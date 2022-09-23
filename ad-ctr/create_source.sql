CREATE SOURCE ad_event (
    event_type VARCHAR,
    bid_id BIGINT,
    ad_id BIGINT,
    event_timestamp TIMESTAMP
) WITH (
    connector = 'kafka',
    kafka.topic = 'ad_event',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;