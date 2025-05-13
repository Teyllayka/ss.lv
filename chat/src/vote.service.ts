import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { createClient, RedisClientType } from 'redis';

@Injectable()
export class VoteService implements OnModuleInit, OnModuleDestroy {
  private redisClient: RedisClientType;

  async onModuleInit() {
    this.redisClient = createClient({
      url: process.env.REDIS_URL,
    });

    this.redisClient.on('error', (err) =>
      console.error('Redis Client Error', err),
    );

    await this.redisClient.connect();
    console.log('Redis connected successfully');
  }

  async onModuleDestroy() {
    await this.redisClient.disconnect();
  }

  async storeVote(
    dealId: number,
    userId: number,
    vote: boolean,
  ): Promise<void> {
    const key = `deal_votes:${dealId}`;

    const votesData = await this.redisClient.get(key);
    const votes = votesData ? JSON.parse(votesData) : {};

    if (votes[userId] !== undefined) {
      throw new Error('User has already voted');
    }
    votes[userId] = vote;

    await this.redisClient.set(key, JSON.stringify(votes), { EX: 3600 });
  }

  async getVotes(dealId: number): Promise<any> {
    const key = `deal_votes:${dealId}`;
    const votesData = await this.redisClient.get(key);
    return votesData ? JSON.parse(votesData) : {};
  }
}
