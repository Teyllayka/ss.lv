import gql from "graphql-tag";

export const GET_ADVERTS = gql`
  query getAdverts {
    getAdverts {
      id
      location
      price
      title
      createdAt
    }
  }
`;

export const GET_ADVERTS_CATEGORY = gql`
  query getAdvertsByCategory($category: String!) {
    getAdvertsByCategory(category: $category) {
      id
      location
      price
      title
      createdAt
    }
  }
`;

export const GET_ADVERT = gql`
  query getAdvert($id: Int!) {
    getAdvert(id: $id) {
      advert {
        id
        price
        location
        available
        title
        createdAt
        description
        category
      }
      user {
        name
        surname
        id
        phone
        email
      }
    }
  }
`;

export const GET_ADVERT_LOGED = gql`
  query getAdvertLoged($id: Int!, $accessToken: String!) {
    getAdvertLoged(id: $id, accessToken: $accessToken) {
      advert {
        id
        price
        location
        available
        title
        createdAt
        description
        category
        isFavorited
      }
      user {
        name
        surname
        id
        phone
        email
      }
    }
  }
`;

export const GET_FAVORITES = gql`
  query getFavorites($accessToken: String!) {
    getFavorites(accessToken: $accessToken) {
      id
      price
      location
      createdAt
      available
      title
      isFavorited
    }
  }
`;

export const CREATE_ADVERT = gql`
  mutation createAdvert(
    $accessToken: String!
    $price: Float!
    $location: String!
    $title: String!
    $description: String!
    $category: String!
    $data: JSON!
  ) {
    createAdvert(
      accessToken: $accessToken
      price: $price
      location: $location
      title: $title
      description: $description
      category: $category
      data: $data
    ) {
      id
      location
      title
      description
      createdAt
    }
  }
`;

export const ADD_FAVORITE = gql`
  mutation addFavorite($accessToken: String!, $advertId: Int!) {
    addFavorite(accessToken: $accessToken, advertId: $advertId) {
      id
      userId
      advertId
      createdAt
    }
  }
`;

export const REMOVE_FAVORITE = gql`
  mutation removeFavorite($accessToken: String!, $advertId: Int!) {
    removeFavorite(accessToken: $accessToken, advertId: $advertId) {
      id
      userId
      advertId
      createdAt
    }
  }
`;

export const GET_ADVERTS_LOGED = gql`
  query getAdvertsLoged($accessToken: String!) {
    getAdvertsLoged(accessToken: $accessToken) {
      id
      price
      location
      available
      title
      createdAt
      isFavorited
    }
  }
`;
