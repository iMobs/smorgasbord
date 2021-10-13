import { ApolloServer } from 'apollo-server';
import { ApolloGateway } from '@apollo/gateway';

const supergraphSdl = '';

// Initialize an ApolloGateway instance and pass it
// the supergraph schema
const gateway = new ApolloGateway({
  supergraphSdl,
});

// Pass the ApolloGateway to the ApolloServer constructor
const server = new ApolloServer({
  gateway,
});

server.listen().then(({ url }) => {
  console.log(`🚀 Server ready at ${url}`);
});
