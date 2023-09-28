import gql from "graphql-tag";

export const GET_ADVERTS = gql`
  query get_adverts {
    getAdverts {
      id
      location
      price
    }
  }
`;
