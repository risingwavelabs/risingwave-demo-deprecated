package workload

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"time"

	"github.com/brianvoe/gofakeit/v6"
	"golang.org/x/time/rate"
)

type tweetData struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Text      string `json:"text"`
	Lang      string `json:"lang"`
}

type twitterEvent struct {
	Data   tweetData   `json:"data"`
	Author twitterUser `json:"author"`
}

type twitterUser struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Name      string `json:"name"`
	UserName  string `json:"username"`
}

const topicTwitterEvents = "twitter"
const tableTwitterEvents = "twitter"

func (r *twitterEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (user_id, ad_id, click_timestamp, impression_timestamp) values ('%d', '%d', '%s', '%s')",
		tableTwitterEvents, r.UserId, r.AdId, r.ClickTimestamp, r.ImpressionTimestamp)
}

func (r *twitterEvent) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return topicTwitterEvents, data
}

type twitterGen struct {
	faker *gofakeit.Faker
	users []*twitterUser
}

func newTwitterGen() *twitterGen {
	faker := gofakeit.New(0)
	users := make(map[string]*twitterUser)
	for len(users) < 100000 {
		id := faker.DigitN(10)
		if _, ok := users[id]; !ok {
			endYear := time.Now().Year() - 1
			startYear := endYear - rand.Intn(8)

			endTime, _ := time.Parse("2006-01-01", fmt.Sprintf("%d-01-01", endYear))
			startTime, _ := time.Parse("2006-01-01", fmt.Sprintf("%d-01-01", startYear))
			users[id] = &twitterUser{
				CreatedAt: faker.DateRange(startTime, endTime).Format("2020-02-12T17:09:56.000Z"),
				Id:        id,
				Name:      fmt.Sprintf("%s ", faker.Name(), faker.Adverb()),
				UserName:  faker.Username(),
			}
		}
	}
	usersList := []*twitterUser{}
	for _, u := range users {
		usersList = append(usersList, u)
	}
	return &twitterGen{
		faker: faker,
		users: usersList,
	}
}

func (t *twitterGen) generate() twitterEvent {
	id := t.faker.DigitN(19)
	author := t.users[rand.Intn(len(t.users))]

	return twitterEvent{
		Data: tweetData{
			Id: id,
		},
		Author: *author,
	}
}

func LoadTwitterEvents(ctx context.Context, cfg GeneratorConfig, sink Sink) error {
	const layout = "2006-01-02 15:04:05.07"

	if _, ok := sink.(*KafkaSink); ok {
		if err := createRequiredTopics(cfg.Brokers, []string{topicTwitterEvents}); err != nil {
			return err
		}
	}

	gen := newTwitterGen()
	count := int64(0)
	initTime := time.Now()
	prevTime := time.Now()
	rl := rate.NewLimiter(rate.Limit(cfg.Qps), 0) // per second
	for {
		record := gen.generate()
		if err := sink.WriteRecord(ctx, &record); err != nil {
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
