// types.ts
import { Generated } from 'kysely';

export interface Database {
  user: User;
  advert: Advert;
  chat: Chat;
  deal: Deal;
  favorites: Favorites;
  message: Message;
  payment: Payment;
  reviews: Reviews;
  seaql_migrations: SeaqlMigrations;
  specifications: Specifications;
}

export interface User {
  id: Generated<number>;
  created_at: Date;
  updated_at: Date;
  avatar_url?: string | null;
  name?: string | null;
  surname?: string | null;
  company_name?: string | null;
  email?: string | null;
  phone?: string | null;
  telegram_id?: string | null;
  telegram_username?: string | null;
  balance: number;
  password_hash?: string | null;
  email_verified: boolean;
  role: 'A' | 'U' | 'M';
}

export interface Advert {
  id: Generated<number>;
  created_at: Date;
  updated_at: Date;
  available: boolean;
  price: number;
  photo_url: string;
  lat: number;
  lon: number;
  additional_photos: string[]; // array of additional photo URLs
  title: string;
  category: string;
  description: string;
  user_id: number;
  sold_to?: number; // nullable relationship field
  archived: boolean;
  old_price: number;
}

export interface Chat {
  id: Generated<number>;
  advert_id: number;
  participant_id: number;
  created_at: Date;
  updated_at: Date;
  archived: boolean;
}

export interface Deal {
  id: Generated<number>;
  chat_id: number;
  price: number;
  created_at: Date;
  requester_id: number;
  status: string; // enum: start, in_progress, completed, canceled
}

export interface Favorites {
  id: Generated<number>;
  created_at: Date;
  user_id: number;
  advert_id: number;
}

export interface Message {
  id: Generated<number>;
  chat_id: number;
  user_id: number;
  content: string;
  created_at: Date;
}

export interface Payment {
  id: Generated<number>;
  order_id: string;
  user_id: number;
  amount: number;
  status: 'P' | 'C' | 'F'; // status enum from schema: Pending, Completed, Failed
}

export interface Reviews {
  id: Generated<number>;
  created_at: Date;
  user_id: number;
  advert_id: number;
  rating: number;
  message: string;
}

export interface SeaqlMigrations {
  version: string;
  applied_at: number;
}

export interface Specifications {
  id: Generated<number>;
  key: string;
  value: string;
  advert_id: number;
}
