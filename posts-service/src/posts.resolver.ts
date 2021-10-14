import {
  Args,
  Directive,
  Field,
  ID,
  ObjectType,
  Parent,
  Query,
  ResolveField,
  Resolver,
} from '@nestjs/graphql';

@ObjectType()
@Directive('@extends')
@Directive('@key(fields: "id")')
export class User {
  @Field((type) => ID)
  @Directive('@external')
  id: number;

  // @Field((type) => [Post])
  // posts?: Post[];
}

@ObjectType()
@Directive('@key(fields: "id")')
export class Post {
  @Field(() => ID)
  id: number;
  @Field()
  title: string;
  @Field()
  content: string;

  @Field((type) => ID)
  authorId: number;

  @Field((type) => User)
  author?: User;
}

@Resolver((of) => Post)
export class PostsResolver {
  @Query(() => Post)
  getPost(@Args('id') id: number): Post {
    return {
      id: 1,
      title: 'Test Post',
      content: 'This is a test',
      authorId: 1,
    };
  }

  @ResolveField((of) => User)
  author(@Parent() post: Post): any {
    return { __typename: 'User', id: post.authorId };
  }
}
