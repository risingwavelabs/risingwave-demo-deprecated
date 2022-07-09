package main

import (
	"context"
	"datagen/gen"
	"log"
	"os"
	"os/signal"
	"syscall"

	_ "github.com/lib/pq"
	"github.com/urfave/cli"
)

var cfg gen.GeneratorConfig = gen.GeneratorConfig{}

func runCommand() error {
	terminateCh := make(chan os.Signal, 1)
	signal.Notify(terminateCh, os.Interrupt, syscall.SIGTERM)

	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		<-terminateCh
		log.Println("Cancelled")
		cancel()
	}()
	return generateLoad(ctx, cfg)
}

func main() {

	app := &cli.App{
		Commands: []cli.Command{
			{
				Name: "postgres",
				Flags: []cli.Flag{
					cli.StringFlag{
						Name:        "host",
						Usage:       "The host address of the PostgreSQL server",
						Required:    false,
						Value:       "localhost",
						Destination: &cfg.DbHost,
					},
					cli.StringFlag{
						Name:        "db",
						Usage:       "The database where the target table is located",
						Required:    false,
						Value:       "dev",
						Destination: &cfg.Database,
					},
					cli.IntFlag{
						Name:        "port",
						Usage:       "The port of the PostgreSQL server",
						Required:    false,
						Value:       4566,
						Destination: &cfg.DbPort,
					},
					cli.StringFlag{
						Name:        "user",
						Usage:       "The user to Postgres",
						Required:    false,
						Value:       "root",
						Destination: &cfg.DbUser,
					},
				},
				Action: func(c *cli.Context) error {
					cfg.Sink = "postgres"
					return runCommand()
				},
			},
			{
				Name: "kafka",
				Flags: []cli.Flag{
					cli.StringFlag{
						Name:        "brokers",
						Usage:       "Kafka bootstrap brokers to connect to, as a comma separated list",
						Required:    true,
						Destination: &cfg.Brokers,
					},
				},
				Action: func(c *cli.Context) error {
					cfg.Sink = "kafka"
					return runCommand()
				},
				HelpName: "datagen kafka",
			},
			{
				Name: "pulsar",
				Flags: []cli.Flag{
					cli.StringFlag{
						Name:        "brokers",
						Usage:       "Pulsar brokers to connect to, as a comma separated list",
						Required:    true,
						Destination: &cfg.Brokers,
					},
				},
				Action: func(c *cli.Context) error {
					cfg.Sink = "pulsar"
					return runCommand()
				},
				HelpName: "datagen pulsar",
			},
		},
		Flags: []cli.Flag{
			cli.BoolFlag{
				Name:        "print",
				Usage:       "Whether to print the content every event",
				Required:    false,
				Destination: &cfg.PrintInsert,
			},
			cli.IntFlag{
				Name:        "qps",
				Usage:       "Number of messages to send per second",
				Required:    false,
				Value:       1,
				Destination: &cfg.Qps,
			},
			cli.StringFlag{
				Name:        "mode",
				Usage:       "ad-click or ad-ctr",
				Required:    true,
				Destination: &cfg.Mode,
			},
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln(err)
	}
}
