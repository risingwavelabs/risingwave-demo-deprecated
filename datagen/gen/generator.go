package gen

import (
	"context"
	"datagen/sink"
	"datagen/sink/kafka"
	"datagen/sink/kinesis"
	"datagen/sink/postgres"
	"datagen/sink/pulsar"

	"gonum.org/v1/gonum/stat/distuv"
)

type GeneratorConfig struct {
	Postgres postgres.PostgresConfig
	Kafka    kafka.KafkaConfig
	Pulsar   pulsar.PulsarConfig
	Kinesis  kinesis.KinesisConfig

	// Whether to print the content of every event.
	PrintInsert bool
	// The datagen mode, e.g. "ad-ctr".
	Mode string
	// The sink type.
	Sink string
	// The throttled requests-per-second.
	Qps int

	// Whether the tail probability is high.
	// If true, We will use uniform distribution for randomizing values.
	HeavyTail bool
}

type LoadGenerator interface {
	KafkaTopics() []string

	Load(ctx context.Context, outCh chan<- sink.SinkRecord)
}

const RwTimestampLayout = "2006-01-02 15:04:05.07+01:00"

type RandDist interface {
	// Rand returns a random number ranging from [0, max].
	Rand(max float64) float64
}

func NewRandDist(cfg GeneratorConfig) RandDist {
	if cfg.HeavyTail {
		return UniformDist{}
	} else {
		return PoissonDist{}
	}
}

type UniformDist struct{}

func (UniformDist) Rand(max float64) float64 {
	d := distuv.Uniform{
		Min: 0,
		Max: max,
	}
	return d.Rand()
}

// A more real-world distribution. The tail will have lower probability..
type PoissonDist struct{}

func (PoissonDist) Rand(max float64) float64 {
	d := distuv.Poisson{
		Lambda: max / 2,
	}
	return d.Rand()
}
