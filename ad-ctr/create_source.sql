CREATE SOURCE ad_impression (
    bid_id BIGINT,
    ad_id BIGINT,
    impression_timestamp TIMESTAMPTZ
) WITH (
    connector = 'kafka',
    kafka.topic = 'ad_impression',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

CREATE SOURCE ad_click (
    bid_id BIGINT,
    click_timestamp TIMESTAMPTZ
) WITH (
    connector = 'kafka',
    kafka.topic = 'ad_click',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;