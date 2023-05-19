# ibkr-cp-api-client
Rust Client for Interactive Brokers Client Portal API. </br>
Take a look at the tests for usage examples. </br>
To use add this to your Cargo.toml: </br>
```ibkr-cp-api-client = { git = "https://github.com/numberjuani/ibkr-cp-api-client.git"} ``` </br>
Docs: https://interactivebrokers.github.io/cpwebapi/ </br>
Note: Make sure ```listenSsl:false``` in your root/conf.yaml for CP Gateway.
Auth will only work in Mac and Linux, if you want to add Windows, open PR. </br>
