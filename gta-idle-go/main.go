package main

import (
	"fmt"
	"time"

	"github.com/go-vgo/robotgo"
)

const (
	tickTime   = 10 * time.Second
	windowName = "Grand Theft Auto V"
)

func main() {
	fmt.Println("This program will work only when GTA V is in focus.")
	fmt.Println("To turn it off, simply close this window.")

	for range time.Tick(tickTime) {
		if robotgo.GetTitle() == windowName {
			robotgo.ScrollMouse(1, "down")
		}
	}
}
