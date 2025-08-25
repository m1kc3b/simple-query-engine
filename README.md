# Simple Rust Query Engine

## 1. Project Description
This repository hosts a simple, educational project: a minimal query engine built from scratch in Rust. It's designed to process basic `SELECT` queries on an in-memory database.

The core components include:

- A Parser that translates a SQL-like query string into a structured representation (an Abstract Syntax Tree).

- An Executor that runs the parsed query against our data store.

- A lightweight, in-memory data store for basic data management.

## 2. From Naive to Optimized: A Journey in Performance
This project serves a dual purpose. We're starting with a naive implementation—one that's clear and easy to understand but intentionally inefficient. This initial version performs a full, linear scan of the entire dataset for every query.

The journey doesn't end there. Our primary goal is to identify and address performance bottlenecks. By deliberately building a slow version first, we can effectively analyze where the system struggles. This will allow us to implement powerful optimization techniques, such as:

- Indexing: To speed up data retrieval and eliminate full table scans.

- Typed Data: To replace generic strings with efficient data types.

- Query Planning: To execute queries in the most optimal way.

This project is a hands-on exploration of the fundamental concepts behind database query engines, focused on the critical process of performance optimization.