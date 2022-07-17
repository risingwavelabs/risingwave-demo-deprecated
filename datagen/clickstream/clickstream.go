package clickstream

import (
	"context"
	"datagen/gen"
	"datagen/sink"
	"encoding/json"
	"fmt"
	"time"

	"github.com/brianvoe/gofakeit/v6"
)

type userBehavior struct {
	UserId           string `json:"user_id"`
	TargetId         string `json:"target_id"`
	TargetType       string `json:"target_type"`
	EventTimestmap   string `json:"event_timestamp"`
	BehaviorType     string `json:"behavior_type"`
	ParentTargetType string `json:"parent_target_type"`
	ParentTargetId   string `json:"parent_target_id"`
}

func (r *userBehavior) ToPostgresSql() string {
	return fmt.Sprintf(`INSERT INTO %s
(user_id, target_id, target_type, event_timestamp, behavior_type, parent_target_type, parent_target_id)
values ('%s', '%s', '%s', '%s', '%s', '%s', '%s')`,
		"user_behaviors", r.UserId, r.TargetId, r.TargetType, r.EventTimestmap, r.BehaviorType, r.ParentTargetType, r.ParentTargetId)
}

func (r *userBehavior) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "user_behaviors", data
}

type targetType int

var targetTypeToString = []string{
	"thread",
	"comment",
	"user",
}

func (t targetType) String() string {
	return targetTypeToString[int(t)]
}

type clickStreamGen struct {
	faker *gofakeit.Faker
}

func NewClickStreamGen() gen.LoadGenerator {
	return &clickStreamGen{
		faker: gofakeit.New(0),
	}
}

func (g *clickStreamGen) randBehaviorType(t targetType) string {
	switch t {
	case 0: // thread
		behaviors := []string{
			"publish", // Publish a thread.
			"show",
			"upvote",
			"downvote",
			"share",
			"award",
			"save",
		}
		return behaviors[g.faker.IntRange(0, len(behaviors)-1)]
	case 1: // comment
		behaviors := []string{
			"publish", // Publish a comment, the parent target type can be a comment or a thread.
			"upvote",
			"downvote",
			"share",
			"award",
			"save",
		}
		return behaviors[g.faker.IntRange(0, len(behaviors)-1)]
	case 2: // user
		behaviors := []string{
			"show", // View the user profile.
			"follow",
			"share",
			"unfollow",
		}
		return behaviors[g.faker.IntRange(0, len(behaviors)-1)]
	default:
		panic("unexpected target type")
	}
}

func (g *clickStreamGen) generate() sink.SinkRecord {
	// TODO: The overall throughput can be further controlled by a scale factor.
	userId := g.faker.IntRange(0, 10)
	targetId := g.faker.IntRange(0, 1000)
	target := targetType(g.faker.IntRange(0, len(targetTypeToString)-1))
	behavior := g.randBehaviorType(target)
	// NOTE: The generated event might not be realistic, for example, a user is allowed to follow itself,
	// and a user can upvote a not existed thread. Anyway, it's just a simple demo.
	parentTargetId := g.faker.IntRange(0, 1000)
	parentTarget := targetType(g.faker.IntRange(0, len(targetTypeToString)-1))

	return &userBehavior{
		UserId:           fmt.Sprint(userId),
		TargetId:         target.String() + fmt.Sprint(targetId),
		TargetType:       target.String(),
		EventTimestmap:   time.Now().Format(gen.RwTimestampLayout),
		BehaviorType:     behavior,
		ParentTargetType: parentTarget.String(),
		ParentTargetId:   parentTarget.String() + fmt.Sprint(parentTargetId),
	}
}

func (g *clickStreamGen) KafkaTopics() []string {
	return []string{"user_behaviors"}
}

func (g *clickStreamGen) Load(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		record := g.generate()
		select {
		case <-ctx.Done():
			return
		case outCh <- record:
		}
	}
}
