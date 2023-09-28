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
