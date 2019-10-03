package main

import (
	"fmt"
	"log"
	"net/http"
	"strconv"
	"time"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		now := time.Now()
		value := float64(now.Unix()) / 10
		fmt.Fprint(w, strconv.FormatFloat(value, 'f', -1, 64))
	})

	log.Fatal(http.ListenAndServe(":8000", nil))
}
