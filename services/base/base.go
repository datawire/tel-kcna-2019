package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		now := time.Now()
		message := now.Format("2006-01-02 15:04:05")
		fmt.Fprintln(w, message)
	})

	log.Fatal(http.ListenAndServe(":8000", nil))
}
