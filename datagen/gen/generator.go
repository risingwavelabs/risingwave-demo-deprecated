package gen

import (
	"context"
	"datagen/sink"
)

type GeneratorConfig struct {
	// The postgres configurations.
	DbHost   string
	Database string
	DbPort   int
	DbUser   string
	// Whether to print the content of every event.
	PrintInsert bool
	// The datagen mode, e.g. "ad-ctr".
	Mode string
	// The sink type.
	Sink string
	// The throttled requests-per-second.
	Qps int
	// Empty if the sink is a database.
	Brokers string
}

type LoadGenerator interface {
	KafkaTopics() []string

	Load(ctx context.Context, outCh chan<- sink.SinkRecord)
}

const RwTimestampLayout = "2006-01-02 15:04:05.07"
