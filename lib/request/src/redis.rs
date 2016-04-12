//  rpc-perf - RPC Performance Testing
//  Copyright 2015 Twitter, Inc
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#[cfg(feature = "unstable")]
extern crate test;

/// FLUSHALL request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(flushall(), "flushall\r\n");
pub fn flushall() -> String {
    "flushall\r\n".to_owned()
}

#[cfg(feature = "unstable")]
#[bench]
fn flushall_benchmark(b: &mut test::Bencher) {
    b.iter(|| flushall());
}

/// SET request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(set("key", "value"), "set key value\r\n");
pub fn set(key: &str, value: &str) -> String {
    format!("set {} {}\r\n", key, value)
}

#[cfg(feature = "unstable")]
#[bench]
fn set_benchmark(b: &mut test::Bencher) {
    b.iter(|| set("key", "value"));
}

/// HSET request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(hset("hash", "key", "value"), "hset hash key value\r\n");
pub fn hset(hash: &str, key: &str, value: &str) -> String {
    format!("hset {} {} {}\r\n", hash, key, value)
}

#[cfg(feature = "unstable")]
#[bench]
fn hset_benchmark(b: &mut test::Bencher) {
    b.iter(|| hset("hash", "key", "value"));
}

/// GET request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(get("key"), "get key\r\n");
pub fn get(key: &str) -> String {
    format!("get {}\r\n", key)
}

#[cfg(feature = "unstable")]
#[bench]
fn get_benchmark(b: &mut test::Bencher) {
    b.iter(|| get("key"));
}

/// HGET request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(hget("hash", "key"), "hget hash key\r\n");
pub fn hget(hash: &str, key: &str) -> String {
    format!("hget {} {}\r\n", hash, key)
}

#[cfg(feature = "unstable")]
#[bench]
fn hget_benchmark(b: &mut test::Bencher) {
    b.iter(|| hget("hash", "key"));
}

/// DEL request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(del("key"), "del key\r\n");
pub fn del(key: &str) -> String {
    format!("del {}\r\n", key)
}

#[cfg(feature = "unstable")]
#[bench]
fn del_benchmark(b: &mut test::Bencher) {
    b.iter(|| del("key"));
}

/// EXPIRE request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(expire("key", 1000), "expire key 1000\r\n");
pub fn expire(key: &str, seconds: usize) -> String {
    format!("expire {} {}\r\n", key, seconds)
}

#[cfg(feature = "unstable")]
#[bench]
fn expire_benchmark(b: &mut test::Bencher) {
    b.iter(|| expire("key", 1000));
}

/// INCR request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(incr("key"), "incr key\r\n");
pub fn incr(key: &str) -> String {
    format!("incr {}\r\n", key)
}

#[cfg(feature = "unstable")]
#[bench]
fn incr_benchmark(b: &mut test::Bencher) {
    b.iter(|| incr("key"));
}

/// DECR request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(decr("key"), "decr key\r\n");
pub fn decr(key: &str) -> String {
    format!("decr {}\r\n", key)
}

#[cfg(feature = "unstable")]
#[bench]
fn decr_benchmark(b: &mut test::Bencher) {
    b.iter(|| decr("key"));
}

/// APPEND request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(append("key", "value"), "append key value\r\n");
pub fn append(key: &str, value: &str) -> String {
    format!("append {} {}\r\n", key, value)
}

#[cfg(feature = "unstable")]
#[bench]
fn append_benchmark(b: &mut test::Bencher) {
    b.iter(|| append("key", "value"));
}

/// PREPEND request
///
/// # Example
/// ```
/// # use rpcperf_request::redis::*;
///
/// assert_eq!(prepend("key", "value"), "prepend key value\r\n");
pub fn prepend(key: &str, value: &str) -> String {
    format!("prepend {} {}\r\n", key, value)
}

#[cfg(feature = "unstable")]
#[bench]
fn prepend_benchmark(b: &mut test::Bencher) {
    b.iter(|| prepend("key", "value"));
}
