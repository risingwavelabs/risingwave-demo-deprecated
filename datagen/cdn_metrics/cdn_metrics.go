package cdn_metrics

import (
	"context"
	"datagen/gen"
	"datagen/sink"
)

type cdnMetricsGen struct {
}

func NewCdnMetricsGen() gen.LoadGenerator {
	return &cdnMetricsGen{}
}

func (g *cdnMetricsGen) KafkaTopics() []string {
	return []string{"tcp_metrics", "nics_metrics"}
}

func (g *cdnMetricsGen) Load(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for i := 0; i < 10; i++ { // Assume there are 10 devices
		go func(i int) {
			m := newDeviceTcpMonitor(i)
			m.emulate(ctx, outCh)
		}(i)
		go func(i int) {
			m := newDeviceNicsMonitor(i)
			m.emulate(ctx, outCh)
		}(i)
	}
}
