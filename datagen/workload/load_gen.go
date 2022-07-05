package workload

import (
	"context"
	"datagen/workload/sink"
	"fmt"
	"log"
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

func LoadGen(ctx context.Context, cfg GeneratorConfig) error {
	sinkImpl := sink.Sink(nil)
	err := error(nil)
	if cfg.Sink == "postgres" {
		sinkImpl, err = sink.OpenPostgresSink(sink.PostgresConfig{
			DbHost:   cfg.DbHost,
			DbPort:   cfg.DbPort,
			DbUser:   cfg.DbUser,
			Database: cfg.Database,
		})
	} else if cfg.Sink == "kafka" {
		sinkImpl, err = sink.OpenKafkaSink(ctx, cfg.Brokers)
	} else if cfg.Sink == "pulsar" {
		sinkImpl, err = sink.OpenPulsarSink(ctx, cfg.Brokers)
	} else {
		return fmt.Errorf("invalid sink type: %s", cfg.Sink)
	}
	if err != nil {
		return err
	}
	defer func() {
		if err = sinkImpl.Close(); err != nil {
			log.Print(err)
		}
	}()
	if cfg.Mode == "ad-click" {
		return LoadAdClick(ctx, cfg, sinkImpl)
	} else if cfg.Mode == "ad-ctr" {
		return nil // TODO
	} else if cfg.Mode == "twitter" {
		return LoadTwitterEvents(ctx, cfg, sinkImpl)
	} else {
		return fmt.Errorf("invalid mode: %s", cfg.Mode)
	}
}

const rwTimestampLayout = "2006-01-02 15:04:05.07"
