# Chaos-Reigns
Ever look at Istio documentation and you are like cool, I just need to update the virtual services and destination rules, re apply, deploy to enable circuit breaking!  How about a/b deploys! Oh right, we love everything automated and no one in their right mind wants to do it by hand using istioctl  

This library has helper functions, so you can make your own Rust CLI tool.  Its as easy as sticking the binary on a cron or in CI/CD, and wield the automated power of Istio.  

# Current Supported Traffic Management:
"fault"
"circuit"

### Fault-Injection
- What is fault injection? 

Fault injection is a system testing method which involves the deliberate introduction of faults and errors into
a system. It can be used to identify design or configuration weaknesses and to ensure that the system 
is able the handle faults and recover from error conditions. Faults can be introduced with compile-time 
injection (modifying the source code of the software) or with runtime injection, in which software triggers 
cause faults during specific scenarios.
  
  - Fault injection explained by https://banzaicloud.com/blog/istio-fault-injection/

### Circuit Breaking
- What is circuit breaking?

Circuit breakers are another useful mechanism Istio provides for creating resilient microservice-based
applications. In a circuit breaker, you set limits for calls to individual hosts within a service, 
such as the number of concurrent connections or how many times calls to this host have failed. 
Once that limit has been reached the circuit breaker “trips” and stops further connections to that
host. Using a circuit breaker pattern enables fast failure rather than clients trying to connect 
to an overloaded or failing host.
 
 - Istio Circuit Breaking https://istio.io/docs/concepts/traffic-management/

# CLI Interface
I would recommend taking a crash course on istio traffic management before trying to use the cli.
Chaos-reigns does not do the deployments for you, but instead updates the Istio CRD's for you in an
automated manner.
```
Eric McBride <ericmcbridedeveloper@gmail.com> github.com/ericmcbride
Reign Chaos onto your Istio Service Mesh

USAGE:
    chaos_reigns [FLAGS] [OPTIONS] --chaos-type <chaos-type> --namespace <namespace> --service <service>

FLAGS:
    -d, --delete     Delete applied rules
    -h, --help       Prints help information
    -r, --random     Randomize the svc and chaos to get attacked
    -V, --version    Prints version information

OPTIONS:
        --base-eject-time <base-eject-time>
            Min amount of time host is ejected for Circuit [env: CHAOS_BASE_EJECT_TIME]  [default: 3m]

    -c, --chaos-type <chaos-type>
            What kind of Istio Fault tolerance is going to be ran [env: CHAOS_TYPE]

        --consecutive-errors <consecutive-errors>
            Number of 5xx before host is ejected for Circuit [env: CHAOS_CONSECUTIVE_ERRORS]  [default: 20]

        --delay <delay>
            Simulated Delay for requests coming to envoy ex: 7s for Global [env: CHAOS_DELAY]  [default: 7s]

        --header-key <header-key>
            Header key to be passed to request in istio [env: CHAOS_HEADER_KEY]

        --header-value <header-value>
            User needs to route via header this is the value [env: CHAOS_HEADER_VALUE]

        --interval <interval>
            Time interval between ejection sweep analysis for Circuit [env: CHAOS_INTERVAL]  [default: 1s]

        --max-connections <max-connections>
            Max TCP Connections for Circuit [env: CHAOS_MAX_CONNECTIONS]  [default: 1]

        --max-eject-percent <max-eject-percent>
            Max % of load balancer pool containers that can be ejected for Circuit [env: CHAOS_MAX_EJECT_PERCENT]
            [default: 100]
        --max-pending-reqs <max-pending-reqs>
            Max Pending HTTP1 Requests for Circuit [env: CHAOS_MAX_PENDING_REQUESTS]  [default: 1]

        --max-requests-per-conn <max-requests-per-conn>
            Setting to 1 disables keep-alive for Circuit [env: CHAOS_MAX_REQUESTS_PER_CONN]  [default: 1]

    -n, --namespace <namespace>
            Kubernetes Namespace for tests to be ran on [env: CHAOS_NAMESPACE]

    -s, --service <service>
            What service should be attacked in the kubernetes namespace [env: CHAOS_SERVICE]

        --traffic-percentage <traffic-percentage>
            Percentage of traffic ex 100 for Global [env: CHAOS_TRAFFIC_PERCENTAGE]  [default: 100]
```

- Example usage for deployment:
```
cargo run -- --service=book-svc --namespace=default --chaos-type=fault
```

- Example usage for deletion:
```
cargo run -- --service=book-svc --namespace=default --delete
```

# TODO:
Everything.  Below is the starting point.
- Find bugs with existing implementations
- Once all other implementations are in, refactor code in a more generic way
- The following chaos-types added: Mirroring, Traffic Shifting, Request Routing
