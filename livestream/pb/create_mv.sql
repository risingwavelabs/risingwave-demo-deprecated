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
    to_timestamp(report_timestamp) as report_timestamp,
    country
FROM
    live_stream_metrics_pb;

CREATE MATERIALIZED VIEW total_user_visit_1min AS
SELECT
    window_start AS report_ts,
    COUNT(DISTINCT user_id) as uv
FROM
    TUMBLE(
        live_stream_metrics,
        report_timestamp,
        INTERVAL '1' MINUTE
    )
GROUP BY
    window_start;