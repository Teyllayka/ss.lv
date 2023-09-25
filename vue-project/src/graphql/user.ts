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
