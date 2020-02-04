```
    __  __                      ___ ____     
   / / / /___  __  ______  ____/ (_) __/_  __
  / /_/ / __ \/ / / / __ \/ __  / / /_/ / / /
 / __  / /_/ / /_/ / / / / /_/ / / __/ /_/ / 
/_/ /_/\____/\__,_/_/ /_/\__,_/_/_/  \__, /  
                                    /____/   
```

![Rust](https://github.com/kevinjqiu/houndify-sdk-rust/workflows/Rust/badge.svg)

# Houndify Rust SDK 
This is a rust library for the [Houndify](https://docs.houndify.com/reference) SDK.
Houndify is a voice AI platform that allows anyone to add smart, conversational interfaces to anything with an internet connection.

# Usage

Add `houndify = "<VERSION>"` to your Cargo.toml file.

See a list of versions [here](https://crates.io/crates/houndify).

## Setup client

```

let client_id = <YOUR_CLIENT_ID>;
let client_key = <YOUR_CLIENT_KEY>;
let api_base = "https://api.houndify.com/";

let client = Client::new(
    api_base,
    client_id,
    client_key,
    None,
);
```

## Create a request info object

```
let request_info = RequestInfo::new();
```

See a list of supported RequestInfo attributes [here](https://docs.houndify.com/reference/RequestInfo).

## Create a voice query object

```
let file = File::open("tests/whattimeisit.wav").unwrap();
let buf = BufReader::new(file);
let query = VoiceQuery::new(Box::new(buf), "kevinq", RequestInfo::new());
```

## Send the voice query and read the result

```
let resp = c.voice_query(query);
match resp {
    Ok(r) => println!("{}", r),
    Err(e) => println!("Error={}", e),
}
```
