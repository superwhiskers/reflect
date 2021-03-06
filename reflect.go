/*

reflect.go -
an implementation of the reflect bot in golang

credits:
  - @hyarsan#3653 - original bot creator

license: gnu agplv3

*/

package main

import (
	// internals
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"os/signal"
	"os/user"
	"regexp"
	"runtime"
	"syscall"
	"time"
	// externals
	"github.com/bwmarrin/discordgo"
	log "github.com/sirupsen/logrus"
	//"github.com/patrickmn/go-cache"
	"github.com/superwhiskers/harmony"
	"github.com/superwhiskers/gopsutil/host"
)

var (
	config       configuration
	handler      *harmony.CommandHandler
	mentionRegex *regexp.Regexp
	escapeRegex  *regexp.Regexp
	hostInfo     *host.InfoStat
	startTime    time.Time
	currentUser  *user.User
	//db *cache.Cache
)

func init() {

	log.SetOutput(os.Stdout)
	log.SetFormatter(&log.TextFormatter{
		DisableColors: true,
	})

}

// the main function for this bot
func main() {

	runtime.GOMAXPROCS(1000)

	startTime = time.Now()

	file, err := os.OpenFile("reflect.log", os.O_WRONLY|os.O_CREATE|os.O_APPEND, 0666)
	if err != nil {

		log.Warnf("unable to open logfile. falling back to stdout-only. error: %v", err)

	} else {

		defer file.Close()
		log.SetOutput(io.MultiWriter(os.Stdout, file))

	}

	cfgByte, err := ioutil.ReadFile("config.json")
	if err != nil {

		log.Fatalf("unable to read config file. error: %v", err)

	}

	err = json.Unmarshal(cfgByte, &config)
	if err != nil {

		log.Fatalf("unable to parse config file as json. error: %v", err)

	}

	hostInfo, err = host.Info()
	if err != nil {

		log.Fatalf("unable to get the host os information. error: %v", err)

	}

	currentUser, err = user.Current()
	if err != nil {

		log.Fatalf("unable to get running user. error: %v", err)

	}

	dg, err := discordgo.New(fmt.Sprintf("Bot %s", config.Token))
	if err != nil {

		log.Fatalf("unable to make a new discordgo session object. error: %v", err)

	}

	mentionRegex = regexp.MustCompile("\\@everyone|\\@here")

	escapeRegex = regexp.MustCompile("\\`|\\*|\\_|\\||\\\\")

	handler = harmony.New(config.Prefix, true)

	registerUtilityCommands()
	registerModCommands()
	registerEvtHandlers()

	dg.AddHandler(handler.OnMessage)

	dg.AddHandler(onReady)

	err = dg.Open()
	if err != nil {

		log.Fatalf("[err]: unable to initiate a websocket session. error: %v", err)

	}

	log.Printf("press ctrl-c to stop the bot...")
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	dg.Close()

	cfgByte, err = json.MarshalIndent(config, "", "	")
	if err != nil {

		log.Fatalf("[err]: unable to convert the config back to json. error: %v", err)

	}

	err = ioutil.WriteFile("config.json", cfgByte, 0644)
	if err != nil {

		log.Fatalf("[err]: unable to output config back to file. error: %v", err)

	}

}
