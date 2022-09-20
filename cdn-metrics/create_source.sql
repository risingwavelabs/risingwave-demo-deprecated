CREATE SOURCE nics_metrics (
    device_id VARCHAR,
    metric_name VARCHAR,
    aggregation VARCHAR,
    nic_name VARCHAR,
    report_time TIMESTAMP,
    bandwidth DOUBLE PRECISION,
    metric_value DOUBLE PRECISION
) WITH (
    connector = 'kafka',
    kafka.topic = 'nics_metrics',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

CREATE SOURCE tcp_metrics (
    device_id VARCHAR,
    metric_name VARCHAR,
    report_time TIMESTAMP,
    metric_value DOUBLE PRECISION
) WITH (
    connector = 'kafka',
    kafka.topic = 'nics_metrics',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;