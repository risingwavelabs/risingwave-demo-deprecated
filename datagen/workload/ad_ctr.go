package workload

import (
	"context"
	"datagen/workload/sink"
	"encoding/json"
	"fmt"
	"log"
	"strconv"
	"time"

	"github.com/brianvoe/gofakeit/v6"
	"golang.org/x/time/rate"
)

type adImpressionEvent struct {
	BidId               int64  `json:"bid_id"`
	AdId                int64  `json:"ad_id"`
	ImpressionTimestamp string `json:"impression_timestamp"`
}

func (r *adImpressionEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (bid_id, ad_id, impression_timestamp) values ('%d', '%d', '%s')",
		"ad_impression", r.BidId, r.AdId, r.ImpressionTimestamp)
}

func (r *adImpressionEvent) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "ad_impression", data
}

type adClickEvent struct {
	BidId          int64  `json:"bid_id"`
	ClickTimestamp string `json:"click_timestamp"`
}

func (r *adClickEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (bid_id, click_timestamp) values ('%d',  '%s')",
		"ad_click", r.BidId, r.ClickTimestamp)
}

func (r *adClickEvent) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "ad_click", data
}

type adCtrGen struct {
	faker *gofakeit.Faker
	ctr   map[int64]float64
}

func newAdCtrGen() *adCtrGen {
	return &adCtrGen{
		ctr:   make(map[int64]float64),
		faker: gofakeit.New(0),
	}
}

func (g *adCtrGen) getCtr(adId int64) float64 {
	if ctr, ok := g.ctr[adId]; ok {
		return ctr
	}
	ctr := g.faker.Float64Range(0, 1)
	g.ctr[adId] = ctr
	return ctr
}

func (g *adCtrGen) hasClick(adId int64) bool {
	return g.faker.Float64Range(0, 1) < g.getCtr(adId)
}

func (g *adCtrGen) generate() []sink.SinkRecord {
	bidId, _ := strconv.ParseInt(g.faker.DigitN(8), 10, 64)
	adId := int64(g.faker.IntRange(1, 10))

	events := []sink.SinkRecord{
		&adImpressionEvent{
			BidId:               bidId,
			AdId:                adId,
			ImpressionTimestamp: time.Now().Format(rwTimestampLayout),
		},
	}
	if g.hasClick(adId) {
		randomDelay := time.Duration(g.faker.IntRange(1, 10) * int(time.Second))
		events = append(events, &adClickEvent{
			BidId:          bidId,
			ClickTimestamp: time.Now().Add(randomDelay).Format(rwTimestampLayout),
		})
	}
	return events
}

func LoadAdCtr(ctx context.Context, cfg GeneratorConfig, snk sink.Sink) error {
	if _, ok := snk.(*sink.KafkaSink); ok {
		if err := sink.CreateRequiredTopics(cfg.Brokers, []string{"ad_click", "ad_impression"}); err != nil {
			return err
		}
	}

	gen := newAdCtrGen()
	count := int64(0)
	initTime := time.Now()
	prevTime := time.Now()
	rl := rate.NewLimiter(rate.Every(time.Second), cfg.Qps) // per second
	for {
		records := gen.generate()
		for _, record := range records {
			if err := snk.WriteRecord(ctx, record); err != nil {
				return err
			}
			_ = rl.Wait(ctx)
			count++
		}
		select {
		case <-ctx.Done():
			return nil
		default:
		}
		if time.Since(prevTime) >= 10*time.Second {
			log.Printf("Sent %d records in total (Elasped: %s)", count, time.Since(initTime).String())
			prevTime = time.Now()
		}
	}
}
