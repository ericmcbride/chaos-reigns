# Chaos-Reigns
Ever look at Istio documentation and you are like cool, I just need to update the virtual services and destination rules, re apply, deploy to enable circuit breaking!  How about a/b deploys! Oh right, we love everything automated and no one in their right mind wants to do it by hand using istioctl  

This library has helper functions, so you can make your own Rust CLI tool.  Its as easy as sticking the binary on a cron or in CI/CD, and wield the automated power of Istio.  

# TODO:
Everything.  Below is the starting point.
- Dynamically Update existing destination rules / virtual services to enable whatever Istio traffic management you need
- Cleanup all rules if certain flag enabled
- Plus a ton more
