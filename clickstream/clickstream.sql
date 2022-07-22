CREATE SOURCE user_behaviors (
    user_id BIGINT,
    target_id VARCHAR,
    target_type VARCHAR,
    event_timestamp TIMESTAMP,
    behavior_type VARCHAR,
    parent_target_type VARCHAR,
    parent_target_id VARCHAR
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'user_behaviors',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
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

CREATE MATERIALIZED VIEW thread_view_count AS WITH t AS (
    SELECT
        target_id,
        COUNT() AS view_count,
        window_start as window_time
    FROM
        TUMBLE(
            user_behaviors,
            event_timestamp,
            INTERVAL '10 minutes'
        )
    WHERE
        target_type = 'thread'
        AND behavior_type = 'show'
    GROUP BY
        target_id,
        window_start
)
SELECT
    target_id,
    SUM(t.view_count) AS view_count,
    window_start as window_time
FROM
    HOP(
        t,
        t.window_time,
        INTERVAL '10 minutes',
        INTERVAL '1440 minutes'
    )
GROUP BY
    target_id,
    window_start;

--- TODO: we need now() for ad-hoc mode.
SELECT
    *
FROM
    thread_view_count
WHERE
    window_time > (
        '2022-7-22 18:43' :: TIMESTAMP - INTERVAL '1 day'
    )
    AND window_time < (
        '2022-7-22 18:43' :: TIMESTAMP - INTERVAL '1 day' + INTERVAL '10 minutes'
    )
    AND target_id = 'thread83';

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