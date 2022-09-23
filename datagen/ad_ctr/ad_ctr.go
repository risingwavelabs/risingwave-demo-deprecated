package ad_ctr

import (
	"context"
	"datagen/gen"
	"datagen/sink"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	"github.com/brianvoe/gofakeit/v6"
)

type adEvent struct {
	EventType         int64              `json:"event_type"`
	AdImpressionEvent *adImpressionEvent `json:"ad_impression_event"`
	AdClickEvent      *adClickEvent      `json:"ad_click_event"`
}

func (r *adEvent) ToPostgresSql() string {
	return ""
}

func (r *adEvent) ToKafka() (topic string, key string, data []byte) {
	data, _ = json.Marshal(r)
	if r.EventType == 1 {
		return "ad", fmt.Sprint(r.AdImpressionEvent.BidId), data
	} else {
		return "ad", fmt.Sprint(r.AdClickEvent.BidId), data
	}
}

type adImpressionEvent struct {
	BidId               int64  `json:"bid_id"`
	AdId                int64  `json:"ad_id"`
	ImpressionTimestamp string `json:"impression_timestamp"`
}

type adClickEvent struct {
	BidId          int64  `json:"bid_id"`
	ClickTimestamp string `json:"click_timestamp"`
}

type adCtrGen struct {
	faker *gofakeit.Faker
	ctr   map[int64]float64
}

func NewAdCtrGen() gen.LoadGenerator {
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
		&adEvent{
			EventType: 1,
			AdImpressionEvent: &adImpressionEvent{
				BidId:               bidId,
				AdId:                adId,
				ImpressionTimestamp: time.Now().Format(gen.RwTimestampLayout),
			},
			AdClickEvent: nil,
		},
	}
	if g.hasClick(adId) {
		randomDelay := time.Duration(g.faker.IntRange(1, 10) * int(time.Second))
		events = append(events, &adEvent{
			EventType:         2,
			AdImpressionEvent: nil,
			AdClickEvent: &adClickEvent{
				BidId:          bidId,
				ClickTimestamp: time.Now().Add(randomDelay).Format(gen.RwTimestampLayout),
			},
		})
	}
	return events
}

func (g *adCtrGen) KafkaTopics() []string {
	return []string{"ad"}
}

func (g *adCtrGen) Load(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		records := g.generate()
		for _, record := range records {
			select {
			case outCh <- record:
			case <-ctx.Done():
				return
			}
		}
	}
}
