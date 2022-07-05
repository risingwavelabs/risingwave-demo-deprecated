package workload

import (
	"context"
	"datagen/workload/sink"
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
const tableAdClicks = "ad_source"

func (r *clickEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (user_id, ad_id, click_timestamp, impression_timestamp) values ('%d', '%d', '%s', '%s')",
		tableAdClicks, r.UserId, r.AdId, r.ClickTimestamp, r.ImpressionTimestamp)
}

func (r *clickEvent) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return topicAdClicks, data
}

func LoadAdClick(ctx context.Context, cfg GeneratorConfig, snk sink.Sink) error {
	const layout = "2006-01-02 15:04:05.07"

	if _, ok := snk.(*sink.KafkaSink); ok {
		if err := sink.CreateRequiredTopics(cfg.Brokers, []string{topicAdClicks}); err != nil {
			return err
		}
	}
	count := int64(0)
	initTime := time.Now()
	prevTime := time.Now()
	rl := rate.NewLimiter(rate.Limit(cfg.Qps), 0) // per second
	for {
		now := time.Now()
		record := clickEvent{
			UserId:              rand.Int63n(100000),
			AdId:                rand.Int63n(10),
			ClickTimestamp:      now.Add(time.Duration(rand.Intn(1000)) * time.Millisecond).Format(layout),
			ImpressionTimestamp: now.Format(layout),
		}
		if err := snk.WriteRecord(ctx, &record); err != nil {
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
