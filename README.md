# tower-ratelimit

This project implements a sliding window counter rate limiting algorithm as described in the
[Cloudflare post "How we built rate limiting capable of scaling to millions of domains"](https://blog.cloudflare.com/counting-things-a-lot-of-different-things/).
Warning: This is a toy project created primarily as an exercise to learn Tower.

## HTTP server example

An example HTTP server using Axum is available in [examples/axum.rs](examples/axum.rs). To run the example:

1. Ensure you have Redis running locally. The easiest way is to use Docker:
    ```sh
    docker run -d -p 6379:6379 redis
    ```
2. Use the following command:
    ```sh
    cargo run --example axum
    ```

## Simulating rate limits

While the server is running, you can generate load using the provided Python script:

```sh
python load.py
```

Press `^C` to stop the script and view the summary.

The load script generates statistics about the requests, such as the distribution of successful and rate-limited responses.
Example output:

```
took 82 seconds
total: 4528 requests

/a
total: 1484 requests
204: 44 responses (3.0%, 0.5 rps, 32.2 rpm)
429: 1440 responses (97.0%, 17.6 rps, 1053.7 rpm)

/b
total: 1542 requests
204: 15 responses (1.0%, 0.2 rps, 11.0 rpm)
429: 1527 responses (99.0%, 18.6 rps, 1117.3 rpm)

/c
total: 1502 requests
204: 87 responses (5.8%, 1.1 rps, 63.7 rpm)
429: 1415 responses (94.2%, 17.3 rps, 1035.4 rpm)
```

In the output above:

* 204 indicates a successful request.
* 429 means the request was rate-limited.

You’ll notice that the allowed request rate is pretty close to what’s configured in the web server:

* `/a` is limited to 5 requests per 10 seconds
* `/b` is limited to 10 requests per minute
* `/c` is limited to 60 requests per minute

## License

The code is licensed under the [Unlicense](LICENSE).
