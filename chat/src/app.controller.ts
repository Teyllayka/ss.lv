import {
  Body,
  Controller,
  Get,
  Param,
  Post,
  Req,
  UseGuards,
} from '@nestjs/common';
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
    return await this.appservice.getMessages(s, b);
  }

  @Post('send-message')
  @UseGuards(AuthGuard)
  async sendMessage(@Req() s, @Body() b) {
    const message = await this.appservice.sendMessage(s, b);
    this.eventsGateway.server.emit('chat-' + message.chat_id, message);
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

    console.log('deal', deal);

    this.eventsGateway.server.emit('deal-' + b.chatId, deal);
    return deal;
  }

  @Get('get-chats')
  @UseGuards(AuthGuard)
  async getChats(@Req() req) {
    return await this.appservice.getChats(req.user);
  }

  @Get('get-last-message/:chatId')
  @UseGuards(AuthGuard)
  async getLastMessage(@Req() s, @Param('chatId') chatId: number) {
    return await this.appservice.getLastMessage(s, chatId);
  }

  @Post('create-chat')
  @UseGuards(AuthGuard)
  async createChat(@Req() s, @Body() b) {
    return await this.appservice.createChatWithoutMessage(s.user.id, b.postId);
  }
}
