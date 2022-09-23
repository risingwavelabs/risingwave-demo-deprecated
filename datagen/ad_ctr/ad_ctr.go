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
	Type           string `json:"event_type"`
	BidId          int64  `json:"bid_id"`
	EventTimestamp string `json:"event_timestamp"`
	AdId           int64  `json:"ad_id"`
}

func (r *adEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (event_type, bid_id, event_timestamp, ad_id) values ('%s', '%d', '%s', '%d')",
		"ad_event", r.Type, r.BidId, r.EventTimestamp, r.AdId)
}

func (r *adEvent) ToKafka() (topic string, key string, data []byte) {
	data, _ = json.Marshal(r)
	return "ad_event", fmt.Sprint(r.BidId), data
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
			BidId:          bidId,
			AdId:           adId,
			EventTimestamp: time.Now().Format(gen.RwTimestampLayout),
			Type:           "impression",
		},
	}
	if g.hasClick(adId) {
		randomDelay := time.Duration(g.faker.IntRange(1, 10) * int(time.Second))
		events = append(events, &adEvent{
			BidId:          bidId,
			AdId:           adId,
			EventTimestamp: time.Now().Add(randomDelay).Format(gen.RwTimestampLayout),
			Type:           "click",
		})
	}
	return events
}

func (g *adCtrGen) KafkaTopics() []string {
	return []string{"ad_event"}
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
