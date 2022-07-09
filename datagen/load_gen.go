package main

import (
	"context"
	"datagen/ad_click"
	"datagen/ad_ctr"
	"datagen/gen"
	"datagen/sink"
	"datagen/twitter"
	"fmt"
	"log"
	"time"

	"go.uber.org/ratelimit"
)

func createSink(ctx context.Context, cfg gen.GeneratorConfig) (sink.Sink, error) {
	if cfg.Sink == "postgres" {
		return sink.OpenPostgresSink(sink.PostgresConfig{
			DbHost:   cfg.DbHost,
			DbPort:   cfg.DbPort,
			DbUser:   cfg.DbUser,
			Database: cfg.Database,
		})
	} else if cfg.Sink == "kafka" {
		return sink.OpenKafkaSink(ctx, cfg.Brokers)
	} else if cfg.Sink == "pulsar" {
		return sink.OpenPulsarSink(ctx, cfg.Brokers)
	} else {
		return nil, fmt.Errorf("invalid sink type: %s", cfg.Sink)
	}
}

// newgen creates a new generator based on the given config.
func newGen(cfg gen.GeneratorConfig) (gen.LoadGenerator, error) {
	if cfg.Mode == "ad-click" {
		return ad_click.NewAdClickGen(), nil
	} else if cfg.Mode == "ad-ctr" {
		return ad_ctr.NewAdCtrGen(), nil
	} else if cfg.Mode == "twitter" {
		return twitter.NewTwitterGen(), nil
	} else {
		return nil, fmt.Errorf("invalid mode: %s", cfg.Mode)
	}
}

// spawnGen spawns one or more goroutines to generate data and send it to outCh.
func spawnGen(ctx context.Context, cfg gen.GeneratorConfig, outCh chan<- sink.SinkRecord) error {
	gen, err := newGen(cfg)
	if err != nil {
		return err
	}
	go gen.Load(ctx, cfg, outCh)
	return nil
}

// generateLoad generates data and sends it to the given sink.
func generateLoad(ctx context.Context, cfg gen.GeneratorConfig) error {
	sinkImpl, err := createSink(ctx, cfg)
	if err != nil {
		return err
	}
	defer func() {
		if err = sinkImpl.Close(); err != nil {
			log.Print(err)
		}
	}()

	outCh := make(chan sink.SinkRecord, 1000)
	if err := spawnGen(ctx, cfg, outCh); err != nil {
		return err
	}

	count := int64(0)
	initTime := time.Now()
	prevTime := time.Now()
	rl := ratelimit.New(cfg.Qps, ratelimit.WithoutSlack) // per second
	for {
		select {
		case <-ctx.Done():
			return nil
		case record := <-outCh:
			// Consume records from the channel and send to sink.
			if err := sinkImpl.WriteRecord(ctx, record); err != nil {
				return err
			}
			_ = rl.Take()
			count++
			if time.Since(prevTime) >= 10*time.Second {
				log.Printf("Sent %d records in total (Elasped: %s)", count, time.Since(initTime).String())
				prevTime = time.Now()
			}
		}
	}
}
