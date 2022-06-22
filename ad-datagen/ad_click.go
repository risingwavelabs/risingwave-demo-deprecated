package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"time"

	"golang.org/x/time/rate"
)

type clickEvent struct {
	UserId              int64  `json:"user_id"`
	AdId                int64  `json:"ad_id"`
	ClickTimestamp      string `json:"click_timestamp"`
	ImpressionTimestamp string `json:"impression_timestamp"`
}

const topicAdClicks = "ad_clicks"

func (r *clickEvent) toPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (user_id, ad_id, click_timestamp, impression_timestamp) values ('%d', '%d', '%s', '%s')",
		"ad_source", r.UserId, r.AdId, r.ClickTimestamp, r.ImpressionTimestamp)
}

func (r *clickEvent) toKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return topicAdClicks, data
}

func loadAdClick(ctx context.Context, sink iSink) error {
	const layout = "2006-01-02 15:04:05.07"

	if err := createRequiredTopics([]string{topicAdClicks}); err != nil {
		return err
	}
	count := int64(0)
	initTime := time.Now()
	prevTime := time.Now()
	rl := rate.NewLimiter(rate.Limit(qps), 0) // per second
	for {
		now := time.Now()
		record := clickEvent{
			UserId:              rand.Int63n(100000),
			AdId:                rand.Int63n(10),
			ClickTimestamp:      now.Add(time.Duration(rand.Intn(1000)) * time.Millisecond).Format(layout),
			ImpressionTimestamp: now.Format(layout),
		}
		if err := sink.writeRecord(ctx, &record); err != nil {
			return err
		}
		_ = rl.Wait(ctx)
		select {
		case <-ctx.Done():
			return nil
		default:
		}
		count++
		if time.Since(prevTime) >= 10*time.Second {
			log.Printf("Sent %d records in total (Elasped: %s)", count, time.Since(initTime).String())
			prevTime = time.Now()
		}
	}
}
