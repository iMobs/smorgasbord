import { ApolloServer } from 'apollo-server';
import { ApolloGateway, RemoteGraphQLDataSource } from '@apollo/gateway';
import { readFileSync } from 'fs';

const supergraphSdl = readFileSync('./supergraph.graphql').toString(); // TODO!

class AuthenticatedDataSource extends RemoteGraphQLDataSource {
  async willSendRequest(options: any) {
    if (options.context.authHeaderValue) {
      options.request.http?.headers.set(
        'Authorization',
        options.context.authHeaderValue,
      );
    }
  }
}

const gateway = new ApolloGateway({
  supergraphSdl,
  buildService: ({ url }) => new AuthenticatedDataSource({ url }),
});

const server = new ApolloServer({
  gateway,
  context: ({ req }) => ({ authHeaderValue: req.headers.authorization }),
});

server
  .listen()
  .then(({ url }) => {
    console.log(`🚀 Gateway ready at ${url}`);
  })
  .catch((err) => {
    console.error(err);
  });
