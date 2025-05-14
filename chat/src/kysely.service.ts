import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { Kysely, PostgresDialect } from 'kysely';
import { Pool } from 'pg';
import { Database } from 'types';

@Injectable()
export class KyselyService implements OnModuleInit, OnModuleDestroy {
  private db: Kysely<Database>;

  constructor() {
    console.log('DB URL:', process.env.DATABASE_URL);
    this.db = new Kysely<Database>({
      dialect: new PostgresDialect({
        pool: new Pool({
          connectionString: process.env.DATABASE_URL,
        }),
      }),
    });
  }

  async onModuleInit() {
    console.log('Kysely connected!');
  }

  async onModuleDestroy() {
    await this.db.destroy();
  }

  getDb() {
    return this.db;
  }
}
