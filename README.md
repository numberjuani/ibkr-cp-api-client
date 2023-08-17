# ibkr-cp-api-client
Rust Client for Interactive Brokers Client Portal API. </br>
Take a look at the tests for usage examples. </br>
To use add this to your Cargo.toml: </br>
```ibkr-cp-api-client = { git = "https://github.com/numberjuani/ibkr-cp-api-client.git"} ``` </br>
[Docs](https://interactivebrokers.github.io/cpwebapi/) </br>
## Note 
Make sure ```listenSsl:false``` in your root/conf.yaml for CP Gateway.

## Features
- `auth` - Authentication, brings in functions that can open and authenticate the portal. Windows not supported, if you want it open a PR.
## Contributing
Contributions welcome from anybody.
1. Branch from main.
2. Open PR.
3. Wait for me to approve.
### Disclaimer
I am not affiliated with interactive brokers in any way nor am I making
any representations about how you should trade or use the code in this repository.
Use at your own risk.
