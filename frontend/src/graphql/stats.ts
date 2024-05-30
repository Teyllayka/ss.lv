import gql from "graphql-tag";

export const STATS = gql`
  query stats {
    stats {
      userCount
      advertCount
      todayUserCount
      todayAdvertCount
    }
  }
`;
