import gql from "graphql-tag";

export const GET_ADVERTS = gql`
  query getAdverts {
    getAdverts {
      id
      location
      price
    }
  }
`;

export const GET_ADVERT = gql`
  query getAdvert($id: Int!) {
    getAdvert(id: $id) {
      id
      price
      location
      available
    }
  }
`;
