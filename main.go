package main

import (
	"flag"
	"k8s.io/client-go/dynamic"
	"k8s.io/client-go/tools/clientcmd"
	"log"
	"os"
	"path/filepath"
)

func main() {
	kubePath, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		log.Fatal(err)
	}

	configPath := kubePath + "/../kube.config"
	kubeConfig := flag.String("kubeConfig", configPath, "Kube config to be used")
	flag.Parse()

	config, err := clientcmd.BuildConfigFromFlags("", *kubeConfig)
	if err != nil {
		log.Fatal(err)
	}

	// create CRD Client for patching virtual services / destination routes
	dynamicClient, err := dynamic.NewForConfig(config)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("Dynamic client %+v", dynamicClient)
}
