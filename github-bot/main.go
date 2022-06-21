package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"syscall"
	"time"

	"github.com/go-resty/resty/v2"
	_ "github.com/lib/pq"
	"github.com/urfave/cli"
)

type Event struct {
	Type     string    `json:"type"`
	CreateAt time.Time `json:"created_at"`
	Id       string    `json:"id"`
	Repo     Repo      `json:"repo"`
}

type Repo struct {
	Name string `json:"name"`
}

// https://docs.github.com/en/rest/activity/events#list-public-events
// Headers:
//   X-Ratelimit-Remaining: [56]
//   X-Ratelimit-Used: [4]
//   X-Poll-Interval: [60]
//   Etag: [W/"1676ebff529a1070d55aa7dfdaccb247cfa9d9dda4158487d055d41e1037f637"]
//   X-Ratelimit-Limit: [60]
func pullEventsToDB(ctx context.Context, db *sql.DB) error {
	token := os.Getenv("GITHUB_TOKEN")
	client := resty.New()

	etag := ""
	for page := 1; ; page++ {
		events := []Event{}
		headers := map[string]string{
			"Accept":        "application/vnd.github.v3+json",
			"Authorization": token,
			"If-None-Match": etag,
		}
		resp, err := client.R().
			SetContext(ctx).
			SetHeaders(headers).
			ForceContentType("application/json").
			SetResult(&events).
			Get("https://api.github.com/events?page=" + strconv.Itoa(page) + "&per_page=100")
		if err != nil {
			if resp.StatusCode() == http.StatusServiceUnavailable || resp.StatusCode() == http.StatusForbidden {
				log.Panicf("github API failed: %s", http.StatusText(resp.StatusCode()))
			}
		}
		if len(events) == 0 {
			break
		}
		for _, e := range events {
			if err := writeEventToDB(ctx, db, e); err != nil {
				return err
			}
		}
		interval, _ := strconv.Atoi(resp.Header()["X-Poll-Interval"][0])
		select {
		case <-ctx.Done():
			return nil
		case <-time.Tick(time.Second * time.Duration(interval)):
		}
		etag = resp.Header()["Etag"][0]
	}
	return nil
}

func writeEventToDB(ctx context.Context, db *sql.DB, e Event) error {
	sql := fmt.Sprintf("INSERT INTO %s (type, created_at, id, repo) values ('%s', '%s', '%s', '%s')",
		table, e.Type, e.CreateAt.Format("2006-01-02 15:04:05"), e.Id, e.Repo.Name)
	if printInsert {
		println(sql)
	}
	_, err := db.ExecContext(ctx, sql)
	return err
}

const table = "github_events"

func runCommand() error {
	terminateCh := make(chan os.Signal, 1)
	signal.Notify(terminateCh, os.Interrupt, syscall.SIGTERM)

	db, err := sql.Open("postgres", fmt.Sprintf("postgresql://root:@%s:%d/%s?sslmode=disable", dbHost, dbPort, database))
	if err != nil {
		return err
	}
	defer func() {
		if err := db.Close(); err != nil {
			log.Panic(err)
		}
	}()

	ctx, cancel := context.WithCancel(context.Background())
	for {
		select {
		case <-terminateCh:
			log.Println("Cancelled")
			cancel()
			os.Exit(1)
		case <-time.NewTicker(time.Second).C:
			if err := pullEventsToDB(ctx, db); err != nil {
				cancel()
				return err
			}
		}
	}
}

var (
	dbHost      string
	database    string
	dbPort      int
	printInsert bool
)

func main() {
	flags := []cli.Flag{
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
		cli.BoolFlag{
			Name:        "print",
			Usage:       "Whether to print the INSERT query of every event",
			Required:    false,
			Destination: &printInsert,
		},
	}
	app := &cli.App{
		Flags: flags,
		Action: func(c *cli.Context) error {
			return runCommand()
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatalln(err)
	}
}
