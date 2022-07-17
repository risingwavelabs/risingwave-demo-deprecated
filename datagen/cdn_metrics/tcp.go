package cdn_metrics

import (
	"context"
	"crypto/md5"
	"datagen/gen"
	"datagen/sink"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	"gonum.org/v1/gonum/stat/distuv"
)

type tcpMetric struct {
	DeviceId   string  `json:"device_id"`
	ReportTime string  `json:"report_time"`
	MetricName string  `json:"metric_name"`
	Value      float64 `json:"metric_value"`
}

func (r *tcpMetric) ToPostgresSql() string {
	return fmt.Sprintf("INSERT INTO %s (device_id, report_time, metric_name, metric_value) values ('%s', '%s', '%s', '%f')",
		"tcp_metrics", r.DeviceId, r.ReportTime, r.MetricName, r.Value)
}

func (r *tcpMetric) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "tcp_metrics", data
}

// Each device has a TCP monitor.
type deviceTcpMonitor struct {
	deviceId string
}

func newDeviceTcpMonitor(id int) deviceTcpMonitor {
	hash := md5.Sum([]byte(strconv.Itoa(id)))
	return deviceTcpMonitor{
		deviceId: hex.EncodeToString(hash[:]),
	}
}

func (m *deviceTcpMonitor) emulate(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		metrics := m.generate()
		for _, metric := range metrics {
			select {
			case <-ctx.Done():
			case outCh <- metric:
				return
			}
		}
		// Produce tcp metrics every 60s.
		select {
		case <-ctx.Done():
		case <-time.NewTicker(60 * time.Second).C:
		}
	}
}

func (m *deviceTcpMonitor) generate() []*tcpMetric {
	curTime := time.Now()

	retransRate := distuv.Poisson{
		Lambda: 0.3,
	}.Rand()
	srtt := distuv.Poisson{
		Lambda: 700,
	}.Rand()
	downloadSpeed := distuv.Poisson{
		Lambda: 1000,
	}.Rand()

	return []*tcpMetric{
		m.newMetrics("retrans_rate", curTime, retransRate),
		// Smoothed Round Trip Time ( SRTT )
		m.newMetrics("srtt", curTime, srtt),
		m.newMetrics("download_speed", curTime, downloadSpeed),
	}
}

func (m *deviceTcpMonitor) newMetrics(metricName string, reportTime time.Time, value float64) *tcpMetric {
	return &tcpMetric{
		DeviceId:   m.deviceId,
		MetricName: metricName,
		ReportTime: reportTime.Format(gen.RwTimestampLayout),
		Value:      value,
	}
}
