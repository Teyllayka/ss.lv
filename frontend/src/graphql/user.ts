import gql from "graphql-tag";

export const GET_USER = gql`
  query GetUser($id: Int!) {
    getUser(id: $id) {
      phone
      email
      name
      surname
      avatarUrl
      adverts {
        id
        location
        price
        createdAt
        available
        title
        photoUrl
      }
    }
  }
`;

export const REGISTER = gql`
  mutation Register(
    $email: String!
    $surname: String!
    $password: String!
    $name: String!
    $phone: String!
    $image: String!
  ) {
    register(
      email: $email
      surname: $surname
      password: $password
      name: $name
      phone: $phone
      image: $image
    ) {
      id
      name
      email
      surname
      phone
    }
  }
`;

export const LOGIN = gql`
  mutation login($email: String!, $password: String!) {
    login(email: $email, password: $password) {
      refreshToken
      accessToken
    }
  }
`;

export const ME = gql`
  query me($accessToken: String!) {
    me(accessToken: $accessToken) {
      phone
      email
      name
      surname
      phone
      avatarUrl
      adverts {
        id
        price
        location
        createdAt
        available
        title
        photoUrl
      }
    }
  }
`;

export const EDIT = gql`
  mutation edit(
    $accessToken: String!
    $name: String!
    $surname: String!
    $phone: String!
    $password: String!
  ) {
    edit(
      accessToken: $accessToken
      name: $name
      surname: $surname
      phone: $phone
      password: $password
    ) {
      name
      surname
      phone
    }
  }
`;

export const REFRESH = gql`
  mutation refresh($refreshToken: String!) {
    refresh(refreshToken: $refreshToken) {
      refreshToken
      accessToken
    }
  }
`;
