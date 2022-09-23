CREATE MATERIALIZED VIEW ad_ctr AS WITH ad_impression AS (
    SELECT
        *
    FROM
        ad_event
    WHERE
        event_type = 'impression'
),
ad_click AS (
    SELECT
        *
    FROM
        ad_event
    WHERE
        event_type = 'click'
)
SELECT
    ad_clicks.ad_id AS ad_id,
    ad_clicks.clicks_count :: NUMERIC / ad_impressions.impressions_count AS ctr
FROM
    (
        SELECT
            ad_id,
            COUNT(*) AS impressions_count
        FROM
            ad_impression
        GROUP BY
            ad_id
    ) AS ad_impressions
    JOIN (
        SELECT
            ad_id,
            COUNT(*) AS clicks_count
        FROM
            ad_click
        GROUP BY
            ad_id
    ) AS ad_clicks ON ad_impressions.ad_id = ad_clicks.ad_id;

CREATE MATERIALIZED VIEW ad_ctr_5min AS WITH ad_impression AS (
    SELECT
        *
    FROM
        ad_event
    WHERE
        event_type = 'impression'
),
ad_click AS (
    SELECT
        *
    FROM
        ad_event
    WHERE
        event_type = 'click'
)
SELECT
    ac.ad_id AS ad_id,
    ac.clicks_count :: NUMERIC / ai.impressions_count AS ctr,
    ai.window_end AS window_end
FROM
    (
        SELECT
            ad_id,
            COUNT(*) AS impressions_count,
            window_end
        FROM
            TUMBLE(
                ad_impression,
                event_timestamp,
                INTERVAL '5' MINUTE
            )
        GROUP BY
            ad_id,
            window_end
    ) AS ai
    JOIN (
        SELECT
            ad_id,
            COUNT(*) AS clicks_count,
            window_end
        FROM
            TUMBLE(ad_click, event_timestamp, INTERVAL '5' MINUTE)
        GROUP BY
            ad_id,
            window_end
    ) AS ac ON ai.ad_id = ac.ad_id
    AND ai.window_end = ac.window_end;