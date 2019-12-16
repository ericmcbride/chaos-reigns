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

	// Have to make this a flag type so you dont have to deal with converting
	// a kube api.Config object to a *test.Config object
	configPath := kubePath + "/../kube.config"
	kubeConfig := flag.String("kubeConfig", configPath, "Kube config to be used")
	//hostName := flag.String("hostName", "", "host name for the istio files ex. ml-intent-svc")
	//delete := flag.Bool("delete", false, "Delete the VS and DR?")
	//user := flag.String("user", "", "user value for http header matching")
	//version := flag.String("version", "", "version hash for destination routing")
	flag.Parse()

	// create *rest.Config object
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
