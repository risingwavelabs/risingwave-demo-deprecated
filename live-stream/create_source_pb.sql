CREATE SOURCE live_stream_metrics_pb WITH (
    connector = 'kafka',
    kafka.topic = 'live_stream_metrics',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT PROTOBUF MESSAGE 'schema.LiveStreamMetrics' ROW SCHEMA LOCATION 'http://file_server:8080/schema';

CREATE MATERIALIZED VIEW live_stream_metrics AS
SELECT
    client_ip,
    user_agent,
    user_id,
    room_id,
    video_bps,
    video_fps,
    video_rtt,
    video_lost_pps,
    video_longest_freeze_duration,
    video_total_freeze_duration,
    to_timestamp(report_timestamp),
    country
FROM
    live_stream_metrics_pb;