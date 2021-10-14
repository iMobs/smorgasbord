import { Module } from '@nestjs/common';
import { GraphQLFederationModule } from '@nestjs/graphql';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { PostsResolver, User } from './posts.resolver';

@Module({
  imports: [
    GraphQLFederationModule.forRoot({
      autoSchemaFile: true,
      buildSchemaOptions: {
        orphanedTypes: [User],
      },
      sortSchema: true,
    }),
  ],
  controllers: [AppController],
  providers: [AppService, PostsResolver],
})
export class AppModule {}
