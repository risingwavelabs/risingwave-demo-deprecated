package twitter

import (
	"bytes"
	"context"
	"datagen/gen"
	"datagen/sink"
	"datagen/twitter/avro"
	"datagen/twitter/proto"
	"encoding/json"
	"fmt"
	"math/rand"
	"time"

	"github.com/brianvoe/gofakeit/v6"
	protobuf "google.golang.org/protobuf/proto"
)

type tweetData struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Text      string `json:"text"`
	Lang      string `json:"lang"`
}

type twitterEvent struct {
	sink.BaseSinkRecord

	Data   tweetData   `json:"data"`
	Author twitterUser `json:"author"`
}

type twitterUser struct {
	CreatedAt string `json:"created_at"`
	Id        string `json:"id"`
	Name      string `json:"name"`
	UserName  string `json:"username"`
	Followers int    `json:"followers"`
}

func (r *twitterEvent) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (data, author) values (%s, %s);",
		"twitter", r.Data.objectString(), r.Author.objectString())
}

func (r *twitterUser) objectString() string {
	return fmt.Sprintf("('%s'::TIMESTAMP, '%s', '%s', '%s')", r.CreatedAt, r.Id, r.Name, r.UserName)
}

func (r *tweetData) objectString() string {
	return fmt.Sprintf("('%s'::TIMESTAMP, '%s', '%s', '%s')", r.CreatedAt, r.Id, r.Text, r.Lang)
}

func (r *twitterEvent) ToJson() (topic string, key string, data []byte) {
	data, _ = json.Marshal(r)
	return "twitter", r.Data.Id, data
}

func (r *twitterEvent) ToProtobuf() (topic string, key string, data []byte) {
	m := proto.Event{
		Data: &proto.TweetData{
			CreatedAt: r.Data.CreatedAt,
			Id:        r.Data.Id,
			Text:      r.Data.Text,
			Lang:      r.Data.Lang,
		},
		Author: &proto.User{
			CreatedAt: r.Author.CreatedAt,
			Id:        r.Author.Id,
			Name:      r.Author.Name,
			UserName:  r.Author.UserName,
			Followers: int64(r.Author.Followers),
		},
	}
	data, err := protobuf.Marshal(&m)
	if err != nil {
		panic(err)
	}
	return "twitter", r.Data.Id, data
}

func (r *twitterEvent) ToAvro() (topic string, key string, data []byte) {
	obj := avro.Event{
		Data: avro.TweetData{
			Created_at: r.Data.CreatedAt,
			Id:         r.Data.Id,
			Text:       r.Data.Text,
			Lang:       r.Data.Lang,
		},
		Author: avro.User{
			Created_at: r.Author.CreatedAt,
			Id:         r.Author.Id,
			Name:       r.Author.Name,
			Username:   r.Author.UserName,
			Followers:  int64(r.Author.Followers),
		},
	}
	var buf bytes.Buffer
	err := obj.Serialize(&buf)
	if err != nil {
		panic(err)
	}
	return "twitter", r.Data.Id, buf.Bytes()
}

type twitterGen struct {
	faker *gofakeit.Faker
	users []*twitterUser
}

func NewTwitterGen() gen.LoadGenerator {
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
				CreatedAt: faker.DateRange(startTime, endTime).Format(gen.RwTimestampLayout),
				Id:        id,
				Name:      fmt.Sprintf("%s %s", faker.Name(), faker.Adverb()),
				UserName:  faker.Username(),
				Followers: gofakeit.IntRange(1, 100000),
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

	wordsCnt := t.faker.IntRange(10, 20)
	hashTagsCnt := t.faker.IntRange(0, 2)
	hashTags := ""
	for i := 0; i < hashTagsCnt; i++ {
		hashTags += fmt.Sprintf("#%s ", t.faker.BuzzWord())
	}
	sentence := fmt.Sprintf("%s%s", hashTags, t.faker.Sentence(wordsCnt))
	return twitterEvent{
		Data: tweetData{
			Id:        id,
			CreatedAt: time.Now().Format(gen.RwTimestampLayout),
			Text:      sentence,
			Lang:      gofakeit.Language(),
		},
		Author: *author,
	}
}

func (t *twitterGen) KafkaTopics() []string {
	return []string{"twitter"}
}

func (t *twitterGen) Load(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		record := t.generate()
		select {
		case <-ctx.Done():
			return
		case outCh <- &record:
		}
	}
}
