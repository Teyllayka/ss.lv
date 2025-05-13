import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { AllExceptionsFilter } from './all-exceptions.filter';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.useGlobalFilters(new AllExceptionsFilter());

  app.enableCors({ origin: 'http://localhost:5173' });

  await app.listen(process.env.PORT ?? 4000, process.env.BACKEND_IP);
}
bootstrap();
