package gen

import (
	"context"
	"datagen/sink"
)

type GeneratorConfig struct {
	DbHost      string
	Database    string
	DbPort      int
	DbUser      string
	PrintInsert bool
	Mode        string
	Sink        string
	Qps         int
	Brokers     string
}

type LoadGenerator interface {
	KafkaTopics() []string

	Load(ctx context.Context, cfg GeneratorConfig, outCh chan<- sink.SinkRecord)
}

const RwTimestampLayout = "2006-01-02 15:04:05.07"
