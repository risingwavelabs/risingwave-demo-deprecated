CREATE SOURCE nics_metrics (
    device_id VARCHAR,
    metric_name VARCHAR,
    aggregation VARCHAR,
    nic_name VARCHAR,
    report_time TIMESTAMP,
    bandwidth DOUBLE PRECISION,
    metric_value DOUBLE PRECISION
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'nics_metrics',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
) ROW FORMAT JSON;

CREATE SOURCE tcp_metrics (
    device_id VARCHAR,
    metric_name VARCHAR,
    report_time TIMESTAMP,
    metric_value DOUBLE PRECISION
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'nics_metrics',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
) ROW FORMAT JSON;

CREATE MATERIALIZED VIEW high_util_tcp_metrics AS
SELECT
    tcp.device_id AS device_id,
    tcp.window_end AS window_end,
    tcp.metric_name AS metric_name,
    tcp.metric_value AS metric_value
FROM
    (
        SELECT
            device_id,
            window_end,
            metric_name,
            AVG(metric_value) AS metric_value
        FROM
            TUMBLE(
                tcp_metrics,
                report_time,
                INTERVAL '3' MINUTE
            )
        GROUP BY
            device_id,
            window_end,
            metric_name
    ) AS tcp
    JOIN (
        SELECT
            device_id,
            window_end,
            AVG((metric_value * 8 / 10) / bandwidth) AS avg_util
        FROM
            TUMBLE(
                nics_metrics,
                report_time,
                INTERVAL '3' MINUTE
            )
        WHERE
            metric_name = 'tx_bytes'
            AND aggregation = 'avg'
        GROUP BY
            device_id,
            window_end
    ) AS nic ON tcp.device_id = nic.device_id
    AND tcp.window_end = nic.window_end
WHERE
    avg_util >= 50;

CREATE MATERIALIZED VIEW retrans_incidents AS
SELECT
    device_id,
    window_end AS trigger_time,
    metric_value AS trigger_value
FROM
    high_util_tcp_metrics
WHERE
    metric_name = 'retrans_rate'
    AND metric_value > 0.15;

CREATE MATERIALIZED VIEW rtt_incidents AS
SELECT
    device_id,
    window_end AS trigger_time,
    metric_value AS trigger_value
FROM
    high_util_tcp_metrics
WHERE
    metric_name = 'srtt'
    AND metric_value > 500.0;

CREATE MATERIALIZED VIEW download_incidents AS
SELECT
    device_id,
    window_end AS trigger_time,
    metric_value AS trigger_value
FROM
    high_util_tcp_metrics
WHERE
    metric_name = 'download_speed'
    AND metric_value < 200.0;