package main

import (
	"flag"
	"k8s.io/client-go/dynamic"
	"k8s.io/client-go/tools/clientcmd"
	"log"
)

func main() {
	kubeConfig := flag.String("kubeConfig", "", "Kube config to be used")
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
