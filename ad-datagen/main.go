package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"syscall"

	_ "github.com/lib/pq"
	"github.com/urfave/cli"
)

func runCommand() error {
	terminateCh := make(chan os.Signal, 1)
	signal.Notify(terminateCh, os.Interrupt, syscall.SIGTERM)

	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		<-terminateCh
		log.Println("Cancelled")
		cancel()
	}()
	return loadGen(ctx)
}

var (
	dbHost      string
	database    string
	dbPort      int
	dbUser      string
	printInsert bool
	mode        string
	sink        string
	qps         int
	brokers     string
)

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
						Destination: &dbHost,
					},
					cli.StringFlag{
						Name:        "db",
						Usage:       "The database where the target table is located",
						Required:    false,
						Value:       "dev",
						Destination: &database,
					},
					cli.IntFlag{
						Name:        "port",
						Usage:       "The port of the PostgreSQL server",
						Required:    false,
						Value:       4566,
						Destination: &dbPort,
					},
					cli.StringFlag{
						Name:        "user",
						Usage:       "The user to Postgres",
						Required:    false,
						Value:       "root",
						Destination: &dbUser,
					},
				},
				Action: func(c *cli.Context) error {
					sink = "postgres"
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
						Destination: &brokers,
					},
				},
				Action: func(c *cli.Context) error {
					sink = "kafka"
					return runCommand()
				},
				HelpName: "ad-datagen postgres",
			},
		},
		Flags: []cli.Flag{
			cli.BoolFlag{
				Name:        "print",
				Usage:       "Whether to print the content every event",
				Required:    false,
				Destination: &printInsert,
			},
			cli.IntFlag{
				Name:        "qps",
				Usage:       "Number of messages to send per second",
				Required:    false,
				Value:       1,
				Destination: &qps,
			},
			cli.StringFlag{
				Name:        "mode",
				Usage:       "ad-click or ad-ctr",
				Required:    true,
				Destination: &mode,
			},
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln(err)
	}
}
