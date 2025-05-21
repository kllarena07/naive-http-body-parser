# Naive HTTP Body Parser

I got tired of writing React Native one day and decided to write a naive implementation of an HTTP 1.1 body parser using as much of the Rust standard library as possible.

## How It Works

**1. Parse the HTTP request into a string**

The binary data from a TCP TLS connection is parsed into a string by converting the buffer of data into a string using the standard Rust string methods.

**2. Extract the body from the string**

The body is extracted from the string by:
  1. Extracting the length of the body from the `Content-Length` header (we'll call this `X`)
  2. Splitting the string on the double CRLF sequence that splits the header and the body, and then getting `X` number of bytes specified from the `Content-Length` header.

> NOTE: because of this, the parser will only work with HTTP 1.1 requests and will not handle chunked encoding.

**3. Convert the string body into a JSON object**

The string body is then converted into a JSON object by utilizing the `HashMap` data structure from the standard Rust collections library.
