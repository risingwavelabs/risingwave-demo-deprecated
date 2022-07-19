CREATE SOURCE user_behaviors (
    user_id BIGINT,
    target_id VARCHAR,
    target_type VARCHAR,
    event_timestamp TIMESTAMP,
    behavior_type VARCHAR,
    parent_target_type VARCHAR,
    parent_target_id VARCHAR
) WITH (
    connector = 'kafka',
    kafka.topic = 'user_behaviors',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

CREATE TABLE user_behaviors (
    user_id BIGINT,
    target_id VARCHAR,
    target_type VARCHAR,
    event_timestamp TIMESTAMP,
    behavior_type VARCHAR,
    parent_target_type VARCHAR,
    parent_target_id VARCHAR
);

CREATE MATERIALIZED VIEW thread_view_count AS
SELECT
    target_id,
    COUNT() AS view_count,
    window_start
FROM
    HOP(
        user_behaviors,
        event_timestamp,
        INTERVAL '10 minutes',
        INTERVAL '1440 minutes'
    )
WHERE
    target_type = 'thread'
    AND behavior_type = 'show'
GROUP BY
    target_id,
    window_start;
