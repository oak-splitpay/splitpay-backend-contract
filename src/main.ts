import 'reflect-metadata';
import * as dotenv from 'dotenv';
dotenv.config();
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { config } from './config/config';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  app.setGlobalPrefix('');
  app.enableCors();

  const port = config.app.port;
  await app.listen(port);

  console.log(`Splitpay Backend running on port ${port}`);
  console.log(`Stellar Network: ${config.stellar.network}`);
}

bootstrap().catch(err => {
  console.error('Failed to start backend:', err);
  process.exit(1);
});
