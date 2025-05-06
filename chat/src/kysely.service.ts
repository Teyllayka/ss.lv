import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { Kysely, PostgresDialect, sql } from 'kysely';
import { Pool } from 'pg';
import { Database } from 'types';

@Injectable()
export class KyselyService implements OnModuleInit, OnModuleDestroy {
  private db: Kysely<Database>;

  constructor() {
    this.db = new Kysely<Database>({
      dialect: new PostgresDialect({
        pool: new Pool({
          connectionString: 'postgresql://postgres:password@localhost:5432/ss',
        }),
      }),
    });
  }

  async onModuleInit() {
    console.log('Kysely connected!');
    //await this.createTablesIfNone();
  }

  async onModuleDestroy() {
    await this.db.destroy();
  }

  getDb() {
    return this.db;
  }

  private async createTablesIfNone() {
    console.log('Ensuring tables...');

    // User Table
    await this.db.schema
      .createTable('user')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('name', 'text', (col) => col.notNull())
      .addColumn('password', 'text', (col) => col.notNull())
      .addColumn('email', 'text', (col) => col.unique())
      .addColumn('avatar_url', 'text')
      .addColumn('surname', 'text')
      .addColumn('company_name', 'text')
      .addColumn('phone', 'text')
      .addColumn('telegram_id', 'text')
      .addColumn('telegram_username', 'text')
      .addColumn('balance', 'real', (col) => col.defaultTo(0))
      .addColumn('email_verified', 'boolean', (col) => col.defaultTo(false))
      .addColumn('role', 'text', (col) => col.defaultTo('U'))
      .addCheckConstraint('role_check', sql`role IN ('A', 'U', 'M')`)
      .execute();

    console.log('User table created!');

    // Advert Table (Post)
    await this.db.schema
      .createTable('advert')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('user_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .addColumn('category', 'text')
      .addColumn('title', 'text', (col) => col.notNull())
      .addColumn('description', 'text', (col) => col.notNull())
      .addColumn('created_at', 'timestamp', (col) => col.defaultTo(sql`now()`))
      .addColumn('updated_at', 'timestamp', (col) => col.defaultTo(sql`now()`))
      .addColumn('photo_url', 'text')
      .addColumn('additional_photos', 'jsonb', (col) =>
        col.defaultTo(sql`'[]'::jsonb`),
      )
      .addColumn('available', 'boolean', (col) => col.defaultTo(true))
      .addColumn('price', 'real', (col) => col.notNull())
      .addColumn('lat', 'text')
      .addColumn('lon', 'text')
      .addColumn('archived', 'boolean', (col) => col.defaultTo(false))
      .execute();

    console.log('Advert table created!');

    // Chat Table
    await this.db.schema
      .createTable('chat')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('post_id', 'uuid', (col) =>
        col.notNull().references('advert.id'),
      )
      .addColumn('deal_state', 'text')
      .addColumn('archived', 'boolean', (col) => col.defaultTo(false))
      .execute();

    console.log('Chat table created!');

    // ChatInfo Table
    await this.db.schema
      .createTable('chat_info')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('chat_id', 'uuid', (col) =>
        col.notNull().references('chat.id'),
      )
      .addColumn('post_id', 'uuid', (col) =>
        col.notNull().references('advert.id'),
      )
      .addColumn('user_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .execute();

    console.log('ChatInfo table created!');

    // Deal Table
    await this.db.schema
      .createTable('deal')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('chat_id', 'uuid', (col) =>
        col.notNull().references('chat.id').unique(),
      )
      .addColumn('price', 'text', (col) => col.notNull())
      .addColumn('requester_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .addColumn('votes', 'jsonb', (col) => col.defaultTo(sql`'{}'::jsonb`))
      .execute();

    console.log('Deal table created!');

    // Message Table
    await this.db.schema
      .createTable('message')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('chat_id', 'uuid', (col) =>
        col.notNull().references('chat.id'),
      )
      .addColumn('user_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .addColumn('text', 'text', (col) => col.notNull())
      .execute();

    console.log('Message table created!');

    // Reviews Table
    await this.db.schema
      .createTable('reviews')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('created_at', 'timestamp', (col) => col.defaultTo(sql`now()`))
      .addColumn('user_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .addColumn('advert_id', 'uuid', (col) =>
        col.notNull().references('advert.id').unique(),
      )
      .addColumn('message', 'text', (col) => col.notNull())
      .addColumn('rating', 'integer', (col) => col.notNull())
      .execute();

    console.log('Reviews table created!');

    // Specifications Table
    await this.db.schema
      .createTable('specifications')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('key', 'text', (col) => col.notNull())
      .addColumn('value', 'text', (col) => col.notNull())
      .addColumn('advert_id', 'uuid', (col) =>
        col.notNull().references('advert.id'),
      )
      .execute();

    console.log('Specifications table created!');

    // Favorites Table
    await this.db.schema
      .createTable('favorites')
      .ifNotExists()
      .addColumn('id', 'uuid', (col) => col.primaryKey())
      .addColumn('created_at', 'timestamp', (col) => col.defaultTo(sql`now()`))
      .addColumn('user_id', 'uuid', (col) =>
        col.notNull().references('user.id'),
      )
      .addColumn('advert_id', 'uuid', (col) =>
        col.notNull().references('advert.id'),
      )
      .execute();

    console.log('Favorites table created!');
  }
}
