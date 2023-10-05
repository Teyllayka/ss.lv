import gql from "graphql-tag";

export const GET_USER = gql`
  query GetUser($id: Int!) {
    getUser(id: $id) {
      phone
      email
      name
      surname
      adverts {
        id
        location
        price
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
  ) {
    register(
      email: $email
      surname: $surname
      password: $password
      name: $name
      phone: $phone
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
      adverts {
        id
        price
        location
      }
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
