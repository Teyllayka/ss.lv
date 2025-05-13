import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { JwtModule } from '@nestjs/jwt';
import { jwtConstants } from './constants';
import { EventsGateway } from './events.gateway';
import { KyselyService } from './kysely.service';
import { VoteModule } from './vote.module';
import { ConfigModule } from '@nestjs/config';

@Module({
  imports: [
     ConfigModule.forRoot({
      isGlobal: true,     
      envFilePath: '.env', 
    }),
    JwtModule.register({
      global: true,
      secret: jwtConstants.secret,
      signOptions: { expiresIn: '7d' },
    }),
    VoteModule,
  ],
  controllers: [AppController],
  providers: [AppService, KyselyService, EventsGateway],
})
export class AppModule {}
