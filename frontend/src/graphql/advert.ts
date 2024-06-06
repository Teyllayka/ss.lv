import gql from "graphql-tag";

export const GET_ADVERTS = gql`
  query getAdverts($accessToken: String!, $offset: Int!, $limit: Int!) {
    getAdverts(accessToken: $accessToken, limit: $limit, offset: $offset) {
      id
      location
      price
      title
      createdAt
      isFavorited
      additionalPhotos
      photoUrl
      category
      specs {
        key
        value
      }
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

export const EDIT_ADVERT = gql`
  mutation editAdvert(
    $accessToken: String!
    $id: Int!
    $price: Float!
    $location: String!
    $title: String!
    $description: String!
    $photos: [String]!
  ) {
    editAdvert(
      accessToken: $accessToken
      id: $id
      price: $price
      location: $location
      title: $title
      description: $description
      photos: $photos
    ) {
      id
      location
      title
      description
      createdAt
    }
  }
`;

export const GET_ADVERT = gql`
  query getAdvert($id: Int!, $accessToken: String!) {
    getAdvert(id: $id, accessToken: $accessToken) {
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
        photoUrl
        additionalPhotos
        specs {
          key
          value
        }
      }
      user {
        name
        surname
        id
        phone
        email
      }
      isAdmin
      belongsToUser
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
      photoUrl
    }
  }
`;

export const DELETE_ADVERT = gql`
  mutation deleteAdvert($accessToken: String!, $advertId: Int!) {
    deleteAdvert(accessToken: $accessToken, advertId: $advertId) {
      id
      location
      title
      description
      createdAt
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
    $photos: [String]!
  ) {
    createAdvert(
      accessToken: $accessToken
      price: $price
      location: $location
      title: $title
      description: $description
      category: $category
      photos: $photos
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
