import { Injectable, UnauthorizedException } from '@nestjs/common';
import { KyselyService } from './kysely.service';
import { Kysely, sql } from 'kysely';
import { Database, Advert, Chat } from 'types';
import { VoteService } from './vote.service';

@Injectable()
export class AppService {
  private readonly db: Kysely<Database>;
  constructor(
    private readonly dbService: KyselyService,
    private readonly voteService: VoteService,
  ) {
    this.db = this.dbService.getDb();
  }



  async areUnread(s) {
    const userId = s.user.id;
    const result = await this.db
    .selectFrom('chat')
    .innerJoin('advert', 'advert.id', 'chat.advert_id')
    .innerJoin('message', 'message.chat_id', 'chat.id')
    .where('advert.user_id', '=', userId)
    .where('message.read_at', 'is', null)
    .whereRef('message.user_id', '=', 'chat.participant_id')
    .select(sql`count(distinct chat.id)`.as('count'))
    .executeTakeFirst();

  return Number(result?.count ?? 0);
  }

  async markMessageRead(s, b) {
    const { chatId, messageId } = b;
    if (!chatId || !messageId) {
      throw new UnauthorizedException('No chat ID or message ID provided');
    }

    const chat = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;

    if (!chat) {
      throw new UnauthorizedException('Chat not found');
    }

    await this.db.updateTable('message')
      .set({ read_at: sql`now()` })
      .where('message.id', '=', messageId)
      .where('message.read_at', 'is', null)
      .execute();
  } 
    

  async getLastMessage(s, chatId: number) {
    let chat: Chat | undefined;

    chat = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;

    if (!chat) {
      throw new UnauthorizedException('No chat found');
    }

    const advert = await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', chat.advert_id)
      .executeTakeFirst();
    if (!advert) {
      throw new UnauthorizedException('Advert not found');
    }

    if (
      advert.user_id !== parseInt(s.user.id) &&
      chat.participant_id !== parseInt(s.user.id)
    ) {
      throw new UnauthorizedException(
        'Not authorized to view messages in this chat',
      );
    }

    const lastMessage = await this.db
      .selectFrom('message')
      .selectAll()
      .where('message.chat_id', '=', chatId)
      .orderBy('message.created_at', 'desc')
      .limit(1)
      .executeTakeFirst();
    return lastMessage;
  }

  async sendMessage(s, b) {
    const { content, chatId, urls } = b;


    let user = await this.db
      .selectFrom('user')
      .selectAll()
      .where('user.id', '=', s.user.id)
      .executeTakeFirst();

    if (user.banned) {
      throw new UnauthorizedException('User is banned');
    }

    let chat: Chat | undefined;

    chat = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;

    if (!chat) {
      throw new UnauthorizedException('No chat found');
    }

    let advert = (await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', chat.advert_id)
      .executeTakeFirst()) as unknown as Advert;

    if (advert.archived || !advert || !advert.available) {
      throw new UnauthorizedException('Post archived');
    }

    if (
      chat.participant_id !== parseInt(s.user.id) &&
      advert.user_id !== parseInt(s.user.id)
    ) {
      throw new UnauthorizedException(
        'Not authorized to send messages in this chat',
      );
    }

    let message = await this.db
      .insertInto('message')
      .values({
      chat_id: chat!.id as unknown as number,
      user_id: s.user.id,
      content,
      ...(urls ? { urls } : {}),
      })
      .returningAll()
      .executeTakeFirst();

    return { message, participant: chat.participant_id, creator: advert.user_id };
  }

  async getMessages(s, b) {
    const { chatId } = b;
    if (!chatId) {
      throw new UnauthorizedException('No chat ID provided');
    }

    const chat = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;
    if (!chat) {
      throw new UnauthorizedException('Chat not found');
    }

    const advert = await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', chat.advert_id)
      .executeTakeFirst();
    if (!advert) {
      throw new UnauthorizedException('Advert not found');
    }

    if (
      advert.user_id !== parseInt(s.user.id) &&
      chat.participant_id !== parseInt(s.user.id)
    ) {
      throw new UnauthorizedException(
        'Not authorized to view messages in this chat',
      );
    }

    let readMessages = await this.db
      .updateTable('message')
      .set({ read_at: sql`now()` })
      .where('message.chat_id', '=', chatId)
      .where('message.user_id', '!=', parseInt(s.user.id))
      .where('message.read_at', 'is', null)
      .returningAll()
      .execute();

    const messages = await this.db
      .selectFrom('message')
      .selectAll()
      .where('message.chat_id', '=', chatId)
      .orderBy('message.created_at', 'asc')
      .execute();

    const partnerId =
      advert.user_id === s.user.id ? chat.participant_id : advert.user_id;

    const partner = await this.db
      .selectFrom('user')
      .selectAll()
      .where('user.id', '=', partnerId)
      .executeTakeFirst();

    const deal = await this.db
      .selectFrom('deal')
      .selectAll()
      .where('deal.chat_id', '=', chatId)
      .executeTakeFirst();

    const chatInfo = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;

    let voteCount = 0;
    if (deal) {
      const votes = await this.voteService.getVotes(deal.id);
      voteCount = Object.keys(votes).length;
    }

    return {
      messages,
      readMessages,
      partner,
      deal: deal
        ? {
            ...deal,
            voteCount,
          }
        : null,
      advert,
      chat: chatInfo,
    };
  }

  async getChats(s) {
    const userId = s.id;

    // First, get all the chats with their related data
    const results = await this.db
      .selectFrom('chat')
      .leftJoin('advert', 'advert.id', 'chat.advert_id')
      .leftJoin('user as owner', 'owner.id', 'advert.user_id')
      .leftJoin('user as participant', 'participant.id', 'chat.participant_id')
      .leftJoin('deal', 'deal.chat_id', 'chat.id')
      .select([
        'chat.id as chat_id',
        'chat.advert_id',
        'chat.participant_id',
        'chat.archived',
        'chat.created_at as chat_created_at',
        'chat.updated_at as chat_updated_at',

        'advert.id as advert_id',
        'advert.user_id as advert_owner_id',
        'advert.created_at as advert_created_at',
        'advert.updated_at as advert_updated_at',
        'advert.available',
        'advert.price',
        'advert.photo_url',
        'advert.lat',
        'advert.lon',
        'advert.additional_photos',
        'advert.title',
        'advert.category',
        'advert.description',
        'advert.sold_to',
        'advert.old_price',

        'owner.id as owner_id',
        'owner.created_at as owner_created_at',
        'owner.updated_at as owner_updated_at',
        'owner.avatar_url as owner_avatar_url',
        'owner.name as owner_name',
        'owner.surname as owner_surname',
        'owner.company_name as owner_company_name',
        'owner.email as owner_email',
        'owner.phone as owner_phone',
        'owner.email_verified as owner_email_verified',
        'owner.role as owner_role',

        'participant.id as participant_id',
        'participant.created_at as participant_created_at',
        'participant.updated_at as participant_updated_at',
        'participant.avatar_url as participant_avatar_url',
        'participant.name as participant_name',
        'participant.surname as participant_surname',
        'participant.company_name as participant_company_name',
        'participant.email as participant_email',
        'participant.phone as participant_phone',
        'participant.email_verified as participant_email_verified',
        'participant.role as participant_role',

        'deal.id as deal_id',
        'deal.chat_id as deal_chat_id',
        'deal.price as deal_price',
        'deal.created_at as deal_created_at',
        'deal.requester_id as deal_requester_id',
      ])
      .where((eb) =>
        eb.or([
          eb('chat.participant_id', '=', userId),
          eb('advert.user_id', '=', userId),
        ]),
      )
      .execute();

    const chatIds = results.map((row) => row.chat_id);

    if (chatIds.length === 0) {
      return [];
    }

    const lastMessages = await this.db
      .selectFrom('message')
      .select([
        'message.id as id',
        'message.chat_id',
        'message.user_id',
        'message.content',
        'message.created_at',
        'message.read_at',
      ])
      .where('message.chat_id', 'in', chatIds)
      .distinctOn('message.chat_id')
      .orderBy('message.chat_id')
      .orderBy('message.created_at', 'desc')
      .execute();

    const lastMessageMap = {};
    lastMessages.forEach((message) => {
      lastMessageMap[message.chat_id] = message;
    });

    const structuredChats = results.map((row) => {
      const owner = {
        id: row.owner_id,
        created_at: row.owner_created_at,
        updated_at: row.owner_updated_at,
        avatar_url: row.owner_avatar_url,
        name: row.owner_name,
        surname: row.owner_surname,
        company_name: row.owner_company_name,
        email: row.owner_email,
        phone: row.owner_phone,
        email_verified: row.owner_email_verified,
        role: row.owner_role,
      };

      const participant = {
        id: row.participant_id,
        created_at: row.participant_created_at,
        updated_at: row.participant_updated_at,
        avatar_url: row.participant_avatar_url,
        name: row.participant_name,
        surname: row.participant_surname,
        company_name: row.participant_company_name,
        email: row.participant_email,
        phone: row.participant_phone,
        email_verified: row.participant_email_verified,
        role: row.participant_role,
      };

      const advert = {
        id: row.advert_id,
        created_at: row.advert_created_at,
        updated_at: row.advert_updated_at,
        available: row.available,
        price: row.price,
        photo_url: row.photo_url,
        lat: row.lat,
        lon: row.lon,
        additional_photos: row.additional_photos,
        title: row.title,
        category: row.category,
        description: row.description,
        user_id: row.advert_owner_id,
        sold_to: row.sold_to,
        old_price: row.old_price,
        owner,
      };

      const deal = row.deal_id
        ? {
            id: row.deal_id,
            chat_id: row.deal_chat_id,
            price: row.deal_price,
            created_at: row.deal_created_at,
            requester_id: row.deal_requester_id,
          }
        : null;

      const chat = {
        id: row.chat_id,
        advert_id: row.advert_id,
        participant_id: row.participant_id,
        archived: row.archived,
        created_at: row.chat_created_at,
        updated_at: row.chat_updated_at,
        last_message: lastMessageMap[row.chat_id] || null,
      };

      return {
        chat,
        advert,
        participant,
        deal,
      };
    });

    return structuredChats;
  }

  async requestDeal(s, b) {
    const { price, chatId, state } = b;

    const user = await this.getUser(s.user.id);
    if (!user) throw new UnauthorizedException('No user found');

    const chat = (await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst()) as unknown as Chat;

    if (!chat) throw new UnauthorizedException('No chat found');

    let deal = await this.db
      .selectFrom('deal')
      .selectAll()
      .where('deal.chat_id', '=', chatId)

      .executeTakeFirst();

    const advert = (await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', chat.advert_id)
      .executeTakeFirst()) as unknown as Advert;

    if (
      chat.participant_id !== parseInt(s.user.id) &&
      advert.user_id !== parseInt(s.user.id)
    ) {
      throw new UnauthorizedException('Not authorized to request a deal');
    }

    let newDeal = null;

    switch (state) {
      case 'start':
        newDeal = await this.startDeal(deal, chatId, price, user.id);
        break;
      case 'stop':
        newDeal = await this.stopDeal(deal, chat.advert_id);
        break;
      case 'accept':
        newDeal = await this.acceptDeal(deal, chatId, chat.advert_id, user.id);
        break;
      case 'decline':
        newDeal = await this.declineDeal(deal, user.id);
        break;
      case 'complete':
        newDeal = await this.completeDeal(deal, chatId, chat.advert_id, user);
        break;
      default:
        throw new UnauthorizedException('Invalid deal state');
    }
    return newDeal;
  }

  async startDeal(
    deal: any,
    chatId: number,
    price: string,
    requesterId: number,
  ) {
    if (deal) throw new UnauthorizedException('Deal already active');
    const newDeal = await this.db
      .insertInto('deal')
      .values({
        chat_id: chatId,
        price: parseFloat(price),
        requester_id: requesterId,
        status: 'pending',
      })
      .returningAll()
      .executeTakeFirst();

    return newDeal;
  }

  async stopDeal(deal: any, postId: number) {
    if (!deal) throw new UnauthorizedException('No deal to stop');

    await this.db.deleteFrom('deal').where('deal.id', '=', deal.id).execute();

    await this.db
      .updateTable('advert')
      .set({ available: true })
      .where('advert.id', '=', postId)
      .execute();

    return null;
  }

  async declineDeal(deal: any, userId: number) {
    if (!deal) throw new UnauthorizedException('No deal found');
    if (deal.requester_id === userId) {
      await this.db.deleteFrom('deal').where('deal.id', '=', deal.id).execute();

      return null;
    }
  }

  async acceptDeal(deal: any, chatId: number, postId: number, userId: number) {
    if (!deal) throw new UnauthorizedException('No deal found');
    if (deal.requester_id !== userId) {
      let newDeal = await this.db
        .updateTable('deal')
        .set({ status: 'accepted' })
        .where('deal.id', '=', deal.id)
        .returningAll()
        .executeTakeFirst();

      await this.db
        .deleteFrom('deal')
        .where('chat_id', 'in', (subQuery) =>
          subQuery
            .selectFrom('chat')
            .select('id')
            .where('advert_id', '=', postId)
            .where('id', '!=', chatId),
        )
        .execute();

      await this.db
        .updateTable('chat')
        .set({ archived: true })
        .where('chat.advert_id', '=', postId)
        .where('chat.id', '!=', chatId)
        .execute();

      await this.db
        .updateTable('advert')
        .set({ available: false })
        .where('advert.id', '=', postId)
        .execute();

      return newDeal;
    }
  }

  async completeDeal(deal: any, chatId: number, postId: number, user: any) {
    if (!deal) throw new UnauthorizedException('No deal found');

    const chat = await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.id', '=', chatId)
      .executeTakeFirst();
    if (!chat) throw new UnauthorizedException('Chat not found');

    const post = await this.getPostById(postId);
    if (!post) throw new UnauthorizedException('Post not found');

    const votes = await this.voteService.getVotes(deal.id);

    if (votes[user.id] !== undefined) {
      throw new UnauthorizedException('User already voted');
    }

    await this.voteService.storeVote(deal.id, user.id, true);

    const updatedVotes = await this.voteService.getVotes(deal.id);


    const expectedVotesCount = 2;
    if (Object.keys(updatedVotes).length === expectedVotesCount) {
      const updatedDeal = await this.db
        .updateTable('deal')
        .set({ status: 'completed' })
        .where('deal.id', '=', deal.id)
        .returningAll()
        .executeTakeFirst();

      await this.db
        .updateTable('advert')
        .set({ available: false, sold_to: chat.participant_id })
        .where('advert.id', '=', postId)
        .execute();

      await this.db
        .updateTable('chat')
        .set({ archived: true })
        .where('chat.id', '=', chatId)
        .execute();

      return updatedDeal;
    } else {
    }

    return deal;
  }

  async getUser(userId: number) {
    return await this.db
      .selectFrom('user')
      .selectAll()
      .where('user.id', '=', userId)
      .executeTakeFirst();
  }

  async getPostById(postId: number) {
    return await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', postId)
      .executeTakeFirst();
  }

  validateUserAccess(user: any, post: any, chatId: number) {
    const isOwner = post.user_id === user.id;
    const hasChatAccess = user.chat_info?.some((x: any) =>
      isOwner ? x.chat_id === chatId : x.post_id === post.id,
    );
    if (!hasChatAccess) throw new UnauthorizedException('No chat');
  }

  async getUserWithChatInfo(userId: number) {
    const user = await this.db
      .selectFrom('user')
      .selectAll()
      .where('user.id', '=', userId)
      .executeTakeFirst();
    return user;
  }

  async createChatWithoutMessage(
    userId: number,
    postId: number,
  ): Promise<Chat> {

    const user = await this.db
      .selectFrom('user')
      .selectAll()
      .where('user.id', '=', userId)
      .executeTakeFirst();

    if (user.banned) {
      throw new UnauthorizedException('User is banned');
    }


    const advert = await this.db
      .selectFrom('advert')
      .selectAll()
      .where('advert.id', '=', postId)
      .executeTakeFirst();

    if (!advert) {
      throw new UnauthorizedException('Advert not found');
    }

    if (advert.archived) {
      throw new UnauthorizedException('Advert is archived');
    }

    let existingChat = await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.advert_id', '=', postId)
      .where('chat.participant_id', '=', userId)
      .executeTakeFirst();

    if (existingChat) {
      return existingChat as unknown as Chat;
    }

    await this.db
      .insertInto('chat')
      .values({
        advert_id: postId,
        participant_id: userId,
        archived: false,
      })
      .execute();

    const newChat = await this.db
      .selectFrom('chat')
      .selectAll()
      .where('chat.advert_id', '=', postId)
      .where('chat.participant_id', '=', userId)
      .executeTakeFirst();

    return newChat as unknown as Chat;
  }
}
