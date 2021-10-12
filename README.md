# Smorgasbord
An overly convoluted mix of ideas and technologies

# Table of Contents
- [The Why](#the-why)
- [The Technologies](#the-technologies)

# The Why
Before I went up into the mountains for a much needed vacation, I was reading [a blog](https://romankudryashov.com/blog/2020/12/graphql-rust) about federated GraphQL and Rust microservices.
This got me thinking about a contrived project to work on where I had several microservices in different languages organized in a monorepo and all using GraphQL.

# The technologies
- The auth service responsible for users and roles (Rust)
	- Diesel
	- async-graphql
- The posts service responsible for blog entries and comments (NodeJS)
	- NestJS
- The communications service responsible for emails (Ruby on Rails)
- Docker/Kubernetes
- Postgres/MariaDB
- Apache Kafka
