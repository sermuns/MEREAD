package cmd

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"runtime"
	"time"

	"github.com/spf13/cobra"
)

func serve(bindAddress string, shouldOpenBrowser bool) {
	if shouldOpenBrowser {
		go func() {
			time.Sleep(time.Second)
			openBrowser("http://" + bindAddress)
		}()
	}

	log.Println("Serving on http://" + bindAddress)

	// serve currrent dir
	http.Handle("/", http.FileServer(http.Dir(".")))
	http.ListenAndServe(bindAddress, nil)
}

var rootCmd = &cobra.Command{
	Use:   "MEREAD",
	Short: "A brief description of your application",
	Long: `A longer description that spans multiple lines and likely contains
examples and usage of using your application. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		bindAddress, _ := cmd.Flags().GetString("bind")
		shouldOpenBrowser, _ := cmd.Flags().GetBool("open")
		serve(bindAddress, shouldOpenBrowser)
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().StringP("bind", "b", "localhost:3000", "address to bind to")
	rootCmd.Flags().BoolP("open", "o", false, "open browser")
}

func openBrowser(url string) {
	var err error
	switch runtime.GOOS {
	case "linux":
		err = exec.Command("xdg-open", url).Start()
	case "windows":
		err = exec.Command("rundll32", "url.dll,FileProtocolHandler", url).Start()
	case "darwin":
		err = exec.Command("open", url).Start()
	default:
		err = fmt.Errorf("unsupported platform")
	}
	if err != nil {
		log.Fatal(err)
	}
}
