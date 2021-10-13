# Smorgasbord
An overly convoluted mix of ideas and technologies

# Table of Contents
- [The Why](#the-why)
- [The Technologies](#the-technologies)
	- [The Auth Service](#the-auth-service)
	- [The Posts Service](#the-posts-service)
	- [The Communications Service](#the-communications-service)
	- [The Apollo Gateway](#the-apollo-gateway)
	- [Shared](#shared)

# The Why
Before I went up into the mountains for a much needed vacation, I was reading [a blog](https://romankudryashov.com/blog/2020/12/graphql-rust) about federated GraphQL and Rust microservices.
This got me thinking about a contrived project to work on where I had several microservices in different languages organized in a monorepo and all using GraphQL.

# The Technologies

This is meant to be more diverse than necessary, this could all be done by one backend but that's not the point.

## The Auth Service
Responsible for users and roles written in Rust.
- Diesel
- async-graphql

## The Posts Service
Responsible for blog entries and comments written in NodeJS.
- NestJS

## The Communications Service
Responsible for emails written in Ruby on Rails.

## The Apollo Gateway
Responsible for combining it all together written in NodeJS.

## Shared
- Docker/Kubernetes
- Postgres/MariaDB
- Apache Kafka
