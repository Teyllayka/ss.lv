import { Body, Controller, Get, Post, Req, UseGuards } from '@nestjs/common';
import { AppService } from './app.service';
import { AuthGuard } from './auth.guard';
import { EventsGateway } from './events.gateway';
import { VoteService } from './vote.service';

@Controller()
export class AppController {
  constructor(
    private readonly appservice: AppService,
    private readonly eventsGateway: EventsGateway,
    private readonly voteService: VoteService,
  ) {}

  @Post('get-message')
  @UseGuards(AuthGuard)
  async getMessages(@Req() s, @Body() b) {
    let messages = await this.appservice.getMessages(s, b);

    for (let message of messages.readMessages) {
      this.eventsGateway.server.emit('message-read-chat-' + message.chat_id, message.id);
    }

    return messages;
  }


  @Get('are-unread')
  @UseGuards(AuthGuard)
  async areUnread(@Req() s) {
    const unread = await this.appservice.areUnread(s);
    return unread;
  }


  @Post('send-message')
  @UseGuards(AuthGuard)
  async sendMessage(@Req() s, @Body() b) {
    const { message, creator, participant } = await this.appservice.sendMessage(s, b);
    this.eventsGateway.server.emit('chat-' + message.chat_id, message);

    this.eventsGateway.server.emit('user-' + (creator.toString() === s.user.id ? participant.toString() : creator.toString()), message);

    return message;
  }

  @Post('update-message')
  @UseGuards(AuthGuard)
  async updateMessage(@Req() s, @Body() b) {
    const message = await this.appservice.updateMessage(s, b);
    this.eventsGateway.server.emit('message-' + message.chat_id);
    return message;
  }

  @Post('delete-message')
  @UseGuards(AuthGuard)
  async deleteMessage(@Req() s, @Body() b) {
    const message = await this.appservice.deleteMessage(s, b);
    this.eventsGateway.server.emit('message-' + message.chat_id);
    return message;
  }

  @Post('request-deal')
  @UseGuards(AuthGuard)
  async requestDeal(@Req() s, @Body() b) {
    let deal = await this.appservice.requestDeal(s, b);
    let voteCount = 0;
    if (deal) {
      const votes = await this.voteService.getVotes(deal.id);
      voteCount = Object.keys(votes).length;

      deal = {
        ...deal,
        voteCount,
      };
    }


    this.eventsGateway.server.emit('deal-' + b.chatId, deal);
    return deal;
  }

  @Get('get-chats')
  @UseGuards(AuthGuard)
  async getChats(@Req() req) {
    return await this.appservice.getChats(req.user);
  }

  @Post('create-chat')
  @UseGuards(AuthGuard)
  async createChat(@Req() s, @Body() b) {
    return await this.appservice.createChatWithoutMessage(s.user.id, b.postId);
  }

  @Post('message-read')
  @UseGuards(AuthGuard)
  async handleMessageRead(
    @Req() s, @Body() b
  ) {
    await this.appservice.markMessageRead(s, b);
    this.eventsGateway.server.emit('message-read-chat-' + b.chatId, b.messageId);
  }
}
